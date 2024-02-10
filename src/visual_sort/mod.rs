use leptos::*;
use rand::prelude::SliceRandom;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{AudioContext, CanvasRenderingContext2d, OscillatorNode};

mod bubble;
mod counting;
mod heap;
mod insertion;
mod merge;
mod quick;
mod radix;
mod selection;

pub struct SortParams<'a> {
    pub canvas_ref: &'a NodeRef<html::Canvas>,
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

#[derive(Copy, Clone)]
pub enum Sort {
    Bubble,
    Counting,
    Heap,
    Insertion,
    Merge,
    Radix,
    Quick,
    Selection,
}

impl Sort {
    pub fn name_as_str(&self) -> &'static str {
        match self {
            Self::Bubble => "Bubble Sort",
            Self::Counting => "Counting Sort",
            Self::Heap => "Heapsort",
            Self::Insertion => "Insertion Sort",
            Self::Merge => "Merge Sort",
            Self::Radix => "Radix Sort",
            Self::Quick => "Quicksort",
            Self::Selection => "Selection Sort",
        }
    }

    pub fn route_as_str(&self) -> &'static str {
        match self {
            Self::Bubble => "/bubble",
            Self::Counting => "/counting",
            Self::Heap => "/heap",
            Self::Insertion => "/insertion",
            Self::Merge => "/merge",
            Self::Radix => "/radix",
            Self::Quick => "/quick",
            Self::Selection => "/selection",
        }
    }

    pub fn init(&self, params: SortParams) -> Box<dyn VisualSort> {
        let base = SortBase::new(params);
        match self {
            Self::Bubble => Box::new(bubble::Bubble::new(base)),
            Self::Counting => Box::new(counting::Counting::new(base)),
            Self::Heap => Box::new(heap::Heap::new(base)),
            Self::Insertion => Box::new(insertion::Insertion::new(base)),
            Self::Merge => Box::new(merge::Merge::new(base)),
            Self::Radix => Box::new(radix::Radix::new(base)),
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
    spacing: f64,
    col_width: f64,
    col_height_pct: f64,
}

impl SortBase {
    pub fn new(params: SortParams) -> Self {
        let mut rng = rand::thread_rng();
        let mut data: Vec<usize> = (1..=params.items).collect();
        data.shuffle(&mut rng);
        let len = data.len() as f64;

        let canvas = params
            .canvas_ref
            .get_untracked()
            .expect("canvas should exist");
        let canvas_w = (canvas.client_width() as f64 / len) * len;
        let canvas_h = canvas.client_height() as f64;
        canvas.set_width(canvas_w as u32);
        canvas.set_height(canvas_h as u32);

        let ctx2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .expect("canvas to have 2d context");

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

        // no spacing if low pixel per item
        let spacing = if canvas_w / len > 4.0 { 2.0 } else { 0.0 };

        // how wide can one item be to for all items to fill the canvas, no spacing front or end
        let col_width = (canvas_w + spacing - (spacing * len)) / len;
        let col_height_pct = canvas_h / len;

        Self {
            array_access: params.array_access,
            array_swap: params.array_swap,
            canvas_h,
            canvas_w,
            ctx2d,
            data,
            done: false,
            osc: audio_osc,
            spacing,
            col_width,
            col_height_pct,
        }
    }

    fn draw<F>(&mut self, set_color: F)
    where
        F: Fn(bool, usize) -> &'static str,
    {
        self.ctx2d
            .clear_rect(0.0, 0.0, self.canvas_w, self.canvas_h);
        // draw each item
        for (i, num) in self.data.iter().enumerate() {
            let y = *num as f64 * self.col_height_pct;
            // draw item inside canvas, with width and spacing, no spacing front or end
            let x = i as f64 * (self.col_width + self.spacing);
            self.ctx2d
                .set_fill_style(&JsValue::from(set_color(self.done, i)));
            self.ctx2d.begin_path();
            self.ctx2d.rect(x, self.canvas_h - y, self.col_width, y);
            self.ctx2d.close_path();
            self.ctx2d.fill();
        }
    }

    fn set_freq(&self, value: usize) {
        let start = 200.0;
        let range = 400.0;
        let len = self.data.len() as f32;
        let freq = start + (range / len) * value as f32;
        self.osc.frequency().set_value(freq);
    }
}
