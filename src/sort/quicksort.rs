use crate::BoostrapColor;
use leptos::html::Canvas;
use leptos::*;
use rand::prelude::SliceRandom;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::CanvasRenderingContext2d;
use web_sys::{AudioContext, OscillatorNode};

struct QuickState {
    lo: usize,
    hi: usize,
    pivot: usize,
    i: usize,
}

pub struct Quick {
    access: RwSignal<usize>,
    swap: RwSignal<usize>,
    data: Vec<usize>,
    pub done: bool,
    canvas_w: f64,
    canvas_h: f64,
    ctx2d: CanvasRenderingContext2d,
    pub osc: OscillatorNode,
    pivots: Vec<QuickState>,
}

impl Quick {
    pub fn new(
        canvas_ref: &NodeRef<Canvas>,
        items: usize,
        volume: RwSignal<f32>,
        access: RwSignal<usize>,
        swap: RwSignal<usize>,
    ) -> Self {
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
            access,
            swap,
            data: nums,
            done: false,
            canvas_h,
            canvas_w,
            ctx2d,
            osc: audio_osc,
            pivots: vec![QuickState {
                lo: 0,
                hi,
                pivot: 0,
                i: 0,
            }],
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

        let (curr_pivot, curr_lo, curr_hi, curr_i) = match self.pivots.last() {
            Some(state) => (
                state.pivot.saturating_sub(1),
                state.lo,
                state.hi,
                state.i.saturating_sub(1),
            ),
            None => (0, 0, self.data.len() - 1, 0),
        };

        // draw each item
        for (i, num) in self.data.iter().enumerate() {
            let height = *num as f64 * (self.canvas_h / self.data.len() as f64);
            // draw item inside canvas, with width and spacing, no spacing front or end
            let x = i as f64 * (width + spacing);

            if !self.done && (i == curr_hi || i == curr_lo) {
                self.ctx2d
                    .set_fill_style(&JsValue::from(BoostrapColor::Green.as_str()));
            } else if !self.done && (i == curr_pivot || i == curr_i) {
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
        // continue previous state, or start a new lower or upper half
        let mut state = match self.pivots.pop() {
            Some(data) => data,
            None => {
                self.done = true;
                return;
            }
        };

        // find all less or equal to pivot, return on tick
        for i in state.i..state.hi {
            self.access.update(|n| *n += 1);
            if self.data[i] <= self.data[state.hi] {
                self.swap.update(|n| *n += 1);
                self.data.swap(i, state.pivot);
                self.osc
                    .frequency()
                    .set_value(((450 / self.data.len()) * self.data[state.pivot] + 250) as f32);
                // tick done
                state.pivot += 1;
                state.i = i + 1;
                self.pivots.push(state);
                return;
            }
        }

        // when all less or equal to pivot has been found
        //   move pivot to it's sorted position
        if state.pivot >= self.data.len() {
            state.pivot = self.data.len() - 1;
        }
        self.data.swap(state.hi, state.pivot);
        self.swap.update(|n| *n += 1);

        // add state for upper half of pivot
        if state.pivot + 1 < state.hi {
            self.pivots.push(QuickState {
                lo: state.pivot + 1,
                hi: state.hi,
                pivot: state.pivot + 1,
                i: state.pivot + 1,
            });
        }

        // add state for lower half of pivot
        if state.pivot > 0 && state.lo < state.pivot - 1 {
            self.pivots.push(QuickState {
                lo: state.lo,
                hi: state.pivot - 1,
                pivot: state.lo,
                i: state.lo,
            });
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
