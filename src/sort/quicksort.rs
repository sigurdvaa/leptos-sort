use crate::BoostrapColor;
use leptos::html::Canvas;
use leptos::*;
use rand::prelude::SliceRandom;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::CanvasRenderingContext2d;
use web_sys::{AudioContext, OscillatorNode};

pub struct Quick {
    x: usize,
    y: usize,
    data: Vec<usize>,
    pub done: bool,
    canvas_w: f64,
    canvas_h: f64,
    ctx2d: CanvasRenderingContext2d,
    pub osc: OscillatorNode,
    pivots: Vec<(usize, usize, usize, usize)>,
}

impl Quick {
    pub fn new(canvas_ref: &NodeRef<Canvas>, items: usize, volume: RwSignal<f32>) -> Self {
        let mut rng = rand::thread_rng();
        let mut nums: Vec<usize> = (1..=items).collect();
        nums.shuffle(&mut rng);

        let canvas = canvas_ref.get_untracked().expect("canvas should exist");
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

        create_effect(move |_| audio_gain.gain().set_value(volume.get()));
        let hi = nums.len() - 1;
        Self {
            x: 0,
            y: 0,
            data: nums,
            done: false,
            canvas_h,
            canvas_w,
            ctx2d,
            osc: audio_osc,
            pivots: vec![(0, 0, hi, 0)],
        }
    }

    pub fn draw(&mut self, ticks: usize) {
        for _ in 0..ticks {
            self.update();
        }

        self.ctx2d
            .clear_rect(0.0, 0.0, self.canvas_w, self.canvas_h);

        let spacing = 2.0;
        // how wide can one item be to for all items to fill the canvas, no spacing front or end
        let width =
            (self.canvas_w + spacing - (spacing * self.data.len() as f64)) / self.data.len() as f64;

        // draw each item
        for (i, num) in self.data.iter().enumerate() {
            let height = *num as f64 * (self.canvas_h / self.data.len() as f64);
            // draw item inside canvas, with width and spacing, no spacing front or end
            let x = i as f64 * (width + spacing);
            if self.x < self.data.len() - 1 && i == self.y + 1 {
                self.ctx2d
                    .set_fill_style(&JsValue::from(BoostrapColor::Light.as_str()));
            } else {
                self.ctx2d
                    .set_fill_style(&JsValue::from(BoostrapColor::Red.as_str()));
            }
            self.ctx2d.begin_path();
            self.ctx2d.rect(x, self.canvas_h - height, width, height);
            self.ctx2d.close_path();
            self.ctx2d.fill();
        }
    }

    fn update(&mut self) {
        let (pivot, lo, hi, mut idx) = match self.pivots.pop() {
            Some(data) => data,
            None => {
                self.done = true;
                return;
            }
        };
        for i in lo..hi {
            if self.data[i] > self.data[hi] {
                self.data.swap(i, idx);
                idx += 1;
                if idx >= self.data.len() {
                    idx = self.data.len() - 1;
                }
                self.pivots.push((pivot, i, hi, idx));
                return;
            }
        }
        if lo >= hi {
            return;
        }
        self.data.swap(hi, idx);

        if idx > 0 {
            self.pivots.push((pivot, idx + 1, hi, idx + 1));
        }
        if idx < self.data.len() - 1 {
            self.pivots.push((pivot, lo, idx - 1, idx + 1));
        }
    }
}

#[allow(dead_code)]
fn quicksort_pivot(list: &mut [usize], lo: usize, hi: usize) -> usize {
    let mut idx: usize = lo;

    for i in lo..hi {
        if list[i] <= list[hi] {
            list.swap(i, idx);
            idx += 1;
        }
    }

    if idx >= list.len() {
        idx -= 1;
    }

    list.swap(hi, idx);
    idx
}

#[allow(dead_code)]
fn quicksort(list: &mut [usize], lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }

    let pivot = quicksort_pivot(list, lo, hi);
    if pivot > 0 {
        quicksort(list, lo, pivot - 1);
    }
    quicksort(list, pivot + 1, hi);
}
