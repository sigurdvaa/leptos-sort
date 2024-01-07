use leptos::html::{Button, Canvas};
use leptos::*;
use rand::prelude::SliceRandom;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::CanvasRenderingContext2d;
use web_sys::{AudioContext, AudioDestinationNode, GainNode, OscillatorNode};

type Callback = Rc<RefCell<Closure<dyn FnMut(f64)>>>;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="d-flex flex-row">
            <Sidebar/>
            <Canvas/>
        </div>
    }
}

#[component]
fn Sidebar() -> impl IntoView {
    view! {
        <div class="d-flex flex-column flex-shrink-0 p-3 text-bg-dark" style="width: 280px;">
            <a href="/" class="d-flex align-items-center mb-3 mb-md-0 me-md-auto text-white text-decoration-none">
                <i class="bi bi-filter fs-3 me-2 text-danger"></i>
                <span class="fs-4 text-danger">Sort</span>
            </a>
            <hr/>
            <ul class="nav nav-pills flex-column mb-auto">
                <li class="nav-item">
                    <a href="#" class="nav-link active" aria-current="page">
                        <i class="bi bi-house me-2"></i>
                        Home
                    </a>
                </li>
                <li>
                    <a href="#" class="nav-link text-white">
                        Sort1
                    </a>
                </li>
                <li>
                    <a href="#" class="nav-link text-white">
                        Sort2
                    </a>
                </li>
                <li>
                    <a href="#" class="nav-link text-white">
                        Sort3
                    </a>
                </li>
            </ul>
        </div>
    }
}

struct Audio {
    ctx: AudioContext,
    osc: OscillatorNode,
    gain: GainNode,
}

struct Bubble {
    x: usize,
    y: usize,
    data: Vec<usize>,
    done: bool,
    ctx2d: Option<CanvasRenderingContext2d>,
    audio: Audio,
}

impl Bubble {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut nums: Vec<usize> = (1..=20).collect();
        nums.shuffle(&mut rng);

        let audio_ctx = AudioContext::new().expect("to create audio context");
        let audio_osc = audio_ctx.create_oscillator().expect("to create oscillator");
        let audio_gain = audio_ctx.create_gain().expect("to create gain");
        audio_gain.gain().set_value(0.0);
        audio_osc.frequency().set_value(440.0);
        audio_osc
            .connect_with_audio_node(&audio_gain)
            .expect("audio connect gain");
        audio_gain
            .connect_with_audio_node(&audio_ctx.destination())
            .expect("gain connect destination");

        Self {
            x: 0,
            y: 0,
            data: nums,
            done: false,
            ctx2d: None,
            audio: Audio {
                ctx: audio_ctx,
                osc: audio_osc,
                gain: audio_gain,
            },
        }
    }

    fn draw(&mut self, canvas_w: f64, canvas_h: f64, ticks: usize) {
        for _ in 0..ticks {
            self.update();
        }

        if let Some(ctx) = &self.ctx2d {
            ctx.clear_rect(0.0, 0.0, canvas_w, canvas_h);
            ctx.set_fill_style(&JsValue::from("red"));

            let spacing = 2.0;
            let width = (canvas_w - (spacing * self.data.len() as f64)) / self.data.len() as f64;

            // draw each item
            for (i, num) in self.data.iter().enumerate() {
                let height = *num as f64 * (canvas_h / self.data.len() as f64);
                let x = i as f64 * (spacing + width);
                ctx.begin_path();
                ctx.rect(x + (spacing / 2.0), canvas_h - height, width, height);
                ctx.close_path();
                ctx.fill();
            }
        }
    }

    fn update(&mut self) {
        for x in self.x..self.data.len() {
            self.x = x;
            for y in self.y..self.data.len() - x - 1 {
                self.y = y;
                if self.data[y] > self.data[y + 1] {
                    self.data.swap(y, y + 1);
                    return;
                }
            }
            self.y = 0;
            self.audio.gain.gain().set_value(0.5);
        }
        self.done = true;
    }
}

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

fn quicksort(list: &mut [usize], lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }

    let pivot = quicksort_pivot(list, lo, hi);
    quicksort(list, lo, pivot - 1);
    quicksort(list, pivot + 1, hi);
}

#[component]
fn Canvas() -> impl IntoView {
    let mut bubble = Bubble::new();
    let mut prev_update = 0.0;

    let canvas_w = 600.0;
    let canvas_h = 350.0;
    let canvas_ref = create_node_ref::<Canvas>();
    let btn_ref = create_node_ref::<Button>();
    let window = web_sys::window().unwrap();
    let window_clone = window.clone();
    let draw: Callback = Rc::new(RefCell::new(Closure::new(move |_| ())));
    let draw_clone = draw.clone();
    let document = leptos::document();

    *draw.borrow_mut() = Closure::new(move |prev_end_time| {
        if prev_update == 0.0 {
            let _ = bubble.audio.osc.start();
            prev_update = prev_end_time;
        }

        if bubble.ctx2d.is_none() {
            let canvas = canvas_ref.get_untracked().expect("canvas should exist");
            bubble.ctx2d = Some(
                canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap(),
            );
        }

        let now = document.timeline().current_time().unwrap();
        let delta = now - prev_update;
        let ticks = delta as usize / 50;
        if ticks > 0 {
            bubble.draw(canvas_w, canvas_h, ticks);
            prev_update = now;
        }

        if !bubble.done {
            let _ =
                window_clone.request_animation_frame(draw_clone.borrow().as_ref().unchecked_ref());
        } else {
            let _ = bubble.audio.osc.stop();
            bubble = Bubble::new();
            btn_ref
                .get_untracked()
                .expect("btn should exist")
                .set_disabled(false);
        }
    });

    let draw_to_canvas = move |_| {
        btn_ref
            .get_untracked()
            .expect("btn should exist")
            .set_disabled(true);
        let _ = window.request_animation_frame(draw.borrow().as_ref().unchecked_ref());
    };

    view! {
        <div class="container-fluid my-3">
            <div class="d-flex justify-content-center mb-3">
                <button class="col-2 btn btn-primary" _ref=btn_ref on:click=draw_to_canvas>
                    CanvasDraw
                </button>
            </div>
            <div class="d-flex justify-content-center mb-3">
                <canvas
                    class="border border-danger border-4"
                    width=canvas_w height=canvas_h
                ref=canvas_ref />
            </div>
        </div>
    }
}
