use crate::{BoostrapColor, SortParams, VisualSort};
use leptos::*;
use rand::prelude::SliceRandom;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{AudioContext, CanvasRenderingContext2d, OscillatorNode};

pub struct Heap {
    array_access: RwSignal<usize>,
    array_swap: RwSignal<usize>,
    data: Vec<usize>,
    done: bool,
    canvas_w: f64,
    canvas_h: f64,
    ctx2d: CanvasRenderingContext2d,
    osc: OscillatorNode,
    heaped: bool,
    heap_len: usize,
    x: usize,
    y: usize,
}

impl VisualSort for Heap {
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
            heaped: false,
            heap_len: 0,
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

        // TODO: once started, the canvas won't resize, move calculations outside loop
        // TODO: refactor sorts, common funs should be in mod.rs
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

            if !self.done && i == self.y {
                self.ctx2d
                    .set_fill_style(&JsValue::from(BoostrapColor::Light.as_str()));
            } else if !self.done && i == self.heap_len {
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
        // use self.data as initial unsorted items, heap, and sorted array

        // add items to heap
        if !self.heaped {
            if self.x < self.data.len() {
                // TODO: if data len is greater than 450, the pitch is the same for all values
                self.osc
                    .frequency()
                    .set_value(((450 / self.data.len()) * self.data[self.x] + 250) as f32);
                self.push(self.data[self.x]);
                self.x += 1;
                return;
            }
            self.heaped = true;
        }

        // remove max heap, and add to last pos in array
        // TODO: visualize heapify?
        if let Some(v) = self.pop() {
            self.osc
                .frequency()
                .set_value(((450 / self.data.len()) * v + 250) as f32);
            self.array_swap.update(|n| *n += 1);
            self.data[self.heap_len] = v;
            return;
        }

        self.done = true;
    }
}

impl Heap {
    fn parent(&self, i: usize) -> Option<usize> {
        if i == 0 {
            return None;
        }
        Some((i - 1) / 2)
    }

    fn left_child(&self, i: usize) -> usize {
        i * 2 + 1
    }

    fn right_child(&self, i: usize) -> usize {
        i * 2 + 2
    }

    fn heap_up(&mut self, i: usize) {
        if let Some(p) = self.parent(i) {
            self.array_access.update(|n| *n += 1);
            if self.data[p] < self.data[i] {
                self.array_swap.update(|n| *n += 1);
                self.data.swap(p, i);
                self.heap_up(p);
                self.y = p;
            }
        }
    }

    fn heap_down(&mut self, i: usize) {
        let l = self.left_child(i);
        let r = self.right_child(i);

        if r > self.heap_len {
            return;
        }

        self.array_access.update(|n| *n += 1);
        if self.data[l] > self.data[r] && self.data[i] <= self.data[l] {
            self.array_swap.update(|n| *n += 1);
            self.data.swap(i, l);
            self.heap_down(l);
        } else if self.data[i] <= self.data[r] {
            self.array_swap.update(|n| *n += 1);
            self.data.swap(i, r);
            self.heap_down(r);
        }
    }

    fn push(&mut self, value: usize) {
        self.array_swap.update(|n| *n += 1);
        self.data[self.heap_len] = value;
        self.y = self.heap_len;
        self.heap_up(self.heap_len);
        self.heap_len += 1;
    }

    fn pop(&mut self) -> Option<usize> {
        if self.heap_len == 0 {
            return None;
        }

        let val = Some(self.data[0]);
        self.heap_len -= 1;
        self.data[0] = self.data[self.heap_len];
        self.heap_down(0);
        val
    }
}
