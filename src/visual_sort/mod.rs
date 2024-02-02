use leptos::html::Canvas;
use leptos::*;
use rand::prelude::SliceRandom;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{AudioContext, CanvasRenderingContext2d, OscillatorNode};

mod bubble;
mod heap;
mod insertion;
mod quick;
mod selection;

pub struct SortParams<'a> {
    pub canvas_ref: &'a NodeRef<Canvas>,
    pub items: usize,
    pub volume: RwSignal<f32>,
    pub array_access: RwSignal<usize>,
    pub array_swap: RwSignal<usize>,
}

pub trait VisualSort {
    fn new(base: SortBase) -> Self
    where
        Self: Sized;

    fn done(&self) -> bool;

    fn draw(&mut self, ticks: usize);

    fn osc_stop(&self);

    fn update(&mut self);
}

pub enum Sort {
    Bubble,
    Heap,
    Insertion,
    Quick,
    Selection,
    // TODO: merge sort
}

impl Sort {
    pub fn name_as_str(&self) -> &'static str {
        match self {
            Self::Bubble => "Bubble Sort",
            Self::Heap => "Heapsort",
            Self::Insertion => "Insertion Sort",
            Self::Quick => "Quicksort",
            Self::Selection => "Selection Sort",
        }
    }

    pub fn route_as_str(&self) -> &'static str {
        match self {
            Self::Bubble => "/bubble",
            Self::Heap => "/heap",
            Self::Insertion => "/insertion",
            Self::Quick => "/quick",
            Self::Selection => "/selection",
        }
    }

    pub fn init(&self, params: SortParams) -> Box<dyn VisualSort> {
        let base = SortBase::new(params);
        match self {
            Self::Bubble => Box::new(bubble::Bubble::new(base)),
            Self::Heap => Box::new(heap::Heap::new(base)),
            Self::Insertion => Box::new(insertion::Insertion::new(base)),
            Self::Quick => Box::new(quick::Quick::new(base)),
            Self::Selection => Box::new(selection::Selection::new(base)),
        }
    }
}

pub struct SortBase {
    array_access: RwSignal<usize>,
    array_swap: RwSignal<usize>,
    canvas_h: f64,
    canvas_w: f64,
    ctx2d: CanvasRenderingContext2d,
    data: Vec<usize>,
    done: bool,
    osc: OscillatorNode,
}

impl SortBase {
    pub fn new(params: SortParams) -> Self {
        let mut rng = rand::thread_rng();
        let mut nums: Vec<usize> = (1..=params.items).collect();
        nums.shuffle(&mut rng);

        let canvas = params
            .canvas_ref
            .get_untracked()
            .expect("canvas should exist");
        let canvas_w = canvas.client_width() as f64;
        let canvas_h = canvas.client_height() as f64;
        canvas.set_width(canvas_w as u32);
        canvas.set_height(canvas_h as u32);

        let ctx2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let audio_ctx = AudioContext::new().expect("to create audio context");
        let audio_osc = audio_ctx.create_oscillator().expect("to create oscillator");
        let audio_gain = audio_ctx.create_gain().expect("to create gain");
        audio_gain.gain().set_value(0.0);
        audio_osc
            .connect_with_audio_node(&audio_gain)
            .expect("audio connect gain");
        audio_gain
            .connect_with_audio_node(&audio_ctx.destination())
            .expect("gain connect destination");
        let _ = audio_osc.start();

        create_effect(move |_| audio_gain.gain().set_value(params.volume.get()));

        Self {
            array_access: params.array_access,
            array_swap: params.array_swap,
            canvas_h,
            canvas_w,
            ctx2d,
            data: nums,
            done: false,
            osc: audio_osc,
        }
    }

    fn draw<F>(&mut self, set_color: F)
    where
        F: Fn(bool, usize) -> &'static str,
    {
        self.ctx2d
            .clear_rect(0.0, 0.0, self.canvas_w, self.canvas_h);

        // TODO: once started, the canvas won't resize, move calculations outside loop
        // TODO: make spacing dynamic based on items per pixel (no spacing if too many items)
        let spacing = 2.0;
        // how wide can one item be to for all items to fill the canvas, no spacing front or end
        let width =
            (self.canvas_w + spacing - (spacing * self.data.len() as f64)) / self.data.len() as f64;

        // draw each item
        for (i, num) in self.data.iter().enumerate() {
            let height = *num as f64 * (self.canvas_h / self.data.len() as f64);
            // draw item inside canvas, with width and spacing, no spacing front or end
            let x = i as f64 * (width + spacing);

            self.ctx2d
                .set_fill_style(&JsValue::from(set_color(self.done, i)));

            self.ctx2d.begin_path();
            self.ctx2d.rect(x, self.canvas_h - height, width, height);
            self.ctx2d.close_path();
            self.ctx2d.fill();
        }
    }

    fn set_freq(&self, value: usize) {
        self.osc
            .frequency()
            .set_value(((450 / self.data.len()) * value + 250) as f32);
    }
}
