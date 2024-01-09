use leptos::html::{Button, Canvas};
use leptos::*;
use leptos_router::*;
use rand::prelude::SliceRandom;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::CanvasRenderingContext2d;
use web_sys::{AudioContext, GainNode, OscillatorNode};

type Callback = Rc<RefCell<Closure<dyn FnMut(f64)>>>;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="d-flex flex-row">
                <Sidebar/>
                <Routes>
                    <Route
                        path="/"
                        view=move || view! { <p>Home</p> }
                    />
                    <Route
                        path="/bubblesort"
                        view=move || view! { <Canvas/> }
                    />
                    <Route
                        path="/*"
                        view=move || view! { <p>Not found</p> }
                    />
                </Routes>
            </div>
        </Router>
    }
}

#[component]
fn Sidebar() -> impl IntoView {
    let location = use_location();

    view! {
        <div class="d-flex flex-column flex-shrink-0 p-3 text-bg-dark" style="width: 280px;">
            <a href="/" class="d-flex align-items-center mb-3 mb-md-0 me-md-auto text-white text-decoration-none">
                <i class="bi bi-filter fs-3 me-2 text-danger"></i>
                <span class="fs-4 text-danger">Sort</span>
            </a>
            <hr/>
            <ul class="nav nav-pills flex-column mb-auto">
                <li class="nav-item">
                    <a href="/" class="nav-link text-danger"
                        class:bg-black=move || location.pathname.get() == "/"
                    >
                        <i class="bi bi-house me-2"></i>
                        Home
                    </a>
                </li>
                <li>
                    <a href="/bubblesort" class="nav-link text-danger"
                        class:bg-black=move || location.pathname.get() == "/bubblesort"
                    >
                        <i class="bi bi-chat me-2"></i>
                        Bubble Sort
                    </a>
                </li>
                <li>
                    <a href="/sort2" class="nav-link text-danger"
                        class:bg-black=move || location.pathname.get() == "/sort2"
                    >
                        <i class="bi bi-question-lg me-2"></i>
                        Sort2
                    </a>
                </li>
                <li>
                    <a href="/sort3" class="nav-link text-danger"
                        class:bg-black=move || location.pathname.get() == "/sort3"
                    >
                        <i class="bi bi-question-lg me-2"></i>
                        Sort3
                    </a>
                </li>
            </ul>
        </div>
    }
}

struct Bubble {
    x: usize,
    y: usize,
    data: Vec<usize>,
    done: bool,
    ctx2d: CanvasRenderingContext2d,
    osc: OscillatorNode,
    gain: GainNode,
}

impl Bubble {
    fn new(canvas_ref: &NodeRef<Canvas>) -> Self {
        let mut rng = rand::thread_rng();
        let mut nums: Vec<usize> = (1..=30).collect();
        nums.shuffle(&mut rng);

        let canvas = canvas_ref.get_untracked().expect("canvas should exist");
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

        Self {
            x: 0,
            y: 0,
            data: nums,
            done: false,
            ctx2d,
            osc: audio_osc,
            gain: audio_gain,
        }
    }

    fn draw(&mut self, canvas_w: f64, canvas_h: f64, ticks: usize) {
        self.gain.gain().set_value(0.1);

        for _ in 0..ticks {
            self.update();
        }

        self.ctx2d.clear_rect(0.0, 0.0, canvas_w, canvas_h);
        self.ctx2d.set_fill_style(&JsValue::from("red"));

        let spacing = 2.0;
        let width = (canvas_w - (spacing * self.data.len() as f64)) / self.data.len() as f64;

        // draw each item
        for (i, num) in self.data.iter().enumerate() {
            let height = *num as f64 * (canvas_h / self.data.len() as f64);
            let x = i as f64 * (spacing + width);
            self.ctx2d.begin_path();
            self.ctx2d
                .rect(x + (spacing / 2.0), canvas_h - height, width, height);
            self.ctx2d.close_path();
            self.ctx2d.fill();
        }
    }

    fn update(&mut self) {
        for x in self.x..self.data.len() {
            self.x = x;
            for y in self.y..self.data.len() - x - 1 {
                self.y = y;
                if self.data[y] > self.data[y + 1] {
                    self.data.swap(y, y + 1);
                    self.osc
                        .frequency()
                        .set_value(((500 / self.data.len()) * self.data[y + 1] + 150) as f32);
                    return;
                }
            }
            self.y = 0;
        }
        self.done = true;
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
    quicksort(list, lo, pivot - 1);
    quicksort(list, pivot + 1, hi);
}

#[component]
fn Canvas() -> impl IntoView {
    let mut bubble_holder: Option<Bubble> = None;
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
            prev_update = prev_end_time;
        }

        if bubble_holder.is_none() {
            bubble_holder = Some(Bubble::new(&canvas_ref));
        }

        if let Some(bubble) = bubble_holder.as_mut() {
            let now = document.timeline().current_time().unwrap();
            let delta = now - prev_update;
            let ticks = delta as usize / 25;
            if ticks > 0 {
                bubble.draw(canvas_w, canvas_h, ticks);
                prev_update = now;
            }

            if !bubble.done {
                let _ = window_clone
                    .request_animation_frame(draw_clone.borrow().as_ref().unchecked_ref());
            } else {
                let _ = bubble.osc.stop();
                bubble_holder = Some(Bubble::new(&canvas_ref));
                btn_ref
                    .get_untracked()
                    .expect("btn should exist")
                    .set_disabled(false);
            }
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
                <button class="col-2 btn btn-outline-danger" _ref=btn_ref on:click=draw_to_canvas>
                    Run Bubble Sort
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
