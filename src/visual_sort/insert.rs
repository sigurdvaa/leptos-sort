use crate::BoostrapColor;
use leptos::html::Canvas;
use leptos::*;
use rand::prelude::SliceRandom;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::CanvasRenderingContext2d;
use web_sys::{AudioContext, OscillatorNode};

pub struct Insert {
    access: RwSignal<usize>,
    swap: RwSignal<usize>,
    x: usize,
    y: usize,
    insert: Option<usize>,
    data: Vec<usize>,
    pub done: bool,
    canvas_w: f64,
    canvas_h: f64,
    ctx2d: CanvasRenderingContext2d,
    pub osc: OscillatorNode,
}

impl Insert {
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

        Self {
            access,
            swap,
            x: 1,
            y: 0,
            insert: None,
            data: nums,
            done: false,
            canvas_h,
            canvas_w,
            ctx2d,
            osc: audio_osc,
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

            if !self.done && self.insert.is_some() {
                match self.insert {
                    Some(s) if i == s => self
                        .ctx2d
                        .set_fill_style(&JsValue::from(BoostrapColor::Light.as_str())),
                    _ => self
                        .ctx2d
                        .set_fill_style(&JsValue::from(BoostrapColor::Red.as_str())),
                }
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
        if let Some(x) = self.insert {
            if x > 0 {
                let y = x - 1;
                self.access.update(|n| *n += 1);
                if x > 0 && self.data[x] < self.data[y] {
                    self.swap.update(|n| *n += 1);
                    self.data.swap(x, y);
                    self.insert = Some(y);
                    return;
                }
            }
            self.insert = None;
        };

        for x in self.x..self.data.len() {
            self.x = x;
            let y = x - 1;
            self.access.update(|n| *n += 1);
            if self.data[x] < self.data[y] {
                self.insert = Some(y);
                self.data.swap(x, y);
                self.swap.update(|n| *n += 1);
                self.osc
                    .frequency()
                    .set_value(((450 / self.data.len()) * self.data[y] + 250) as f32);
                self.x = x + 1;
                return;
            }
        }
        self.done = true;
    }
}
