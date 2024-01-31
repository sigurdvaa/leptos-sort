use crate::{BoostrapColor, SortParams, VisualSort};
use leptos::*;
use rand::prelude::SliceRandom;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{AudioContext, CanvasRenderingContext2d, OscillatorNode};

pub struct Selection {
    array_access: RwSignal<usize>,
    array_swap: RwSignal<usize>,
    s: usize,
    x: usize,
    y: usize,
    data: Vec<usize>,
    done: bool,
    canvas_w: f64,
    canvas_h: f64,
    ctx2d: CanvasRenderingContext2d,
    osc: OscillatorNode,
}

impl VisualSort for Selection {
    fn new(params: SortParams) -> Self {
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
            s: 0,
            x: 0,
            y: 0,
            data: nums,
            done: false,
            canvas_h,
            canvas_w,
            ctx2d,
            osc: audio_osc,
        }
    }

    fn done(&self) -> bool {
        self.done
    }

    fn draw(&mut self, ticks: usize) {
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

            if !self.done && i == self.y {
                self.ctx2d
                    .set_fill_style(&JsValue::from(BoostrapColor::Light.as_str()));
            } else if !self.done && (i == self.x || i == self.s) {
                self.ctx2d
                    .set_fill_style(&JsValue::from(BoostrapColor::Green.as_str()));
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

    fn osc_stop(&self) {
        let _ = self.osc.stop();
    }

    fn update(&mut self) {
        if self.x < self.data.len() - 1 {
            if self.y < self.data.len() {
                self.array_access.update(|n| *n += 1);
                if self.data[self.y] < self.data[self.s] {
                    self.s = self.y;
                    self.osc
                        .frequency()
                        .set_value(((450 / self.data.len()) * self.data[self.s] + 250) as f32);
                }
                self.y += 1;
                return;
            }

            self.array_swap.update(|n| *n += 1);
            self.data.swap(self.x, self.s);

            self.x += 1;
            self.s = self.x;
            self.y = self.x + 1;
            self.osc
                .frequency()
                .set_value(((450 / self.data.len()) * self.data[self.s] + 250) as f32);
        } else {
            self.done = true;
        }
    }
}
