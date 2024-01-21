use leptos::html::Canvas;
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
    let update_ms = create_rw_signal(25);
    let play = create_rw_signal(false);
    let items = create_rw_signal(30);
    let volume = create_rw_signal(0.1);
    view! {
        <Router>
            <div class="d-flex flex-row vh-100">
                <Sidebar/>
                <Routes>
                    <Route
                        path="/"
                        view=|| view! { <Home/> }
                    />
                    <Route
                        path="/bubblesort"
                        view=move || view! { <BubbleSort play update_ms items volume/> }
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
        <div class="d-flex flex-column flex-shrink-0 p-3" style="width: 260px;">
            <a href="/" class="d-flex align-items-center ms-3 mb-3 mb-md-0 me-md-auto text-decoration-none">
                <i class="bi bi-filter fs-3 me-2 text-danger"></i>
                <span class="fs-4 text-danger">Sort</span>
            </a>
            <hr/>
            <ul class="nav nav-pills flex-column mb-auto">
                <li class="nav-item">
                    <a href="/" class="nav-link text-white"
                        class:bg-danger=move || location.pathname.get() == "/" >
                        <i class="bi bi-house me-2"></i>
                        Home
                    </a>
                </li>
                <li>
                    <a href="/bubblesort" class="nav-link text-white"
                        class:bg-danger=move || location.pathname.get() == "/bubblesort" >
                        <i class="bi bi-chat me-2"></i>
                        Bubble Sort
                    </a>
                </li>
                <li>
                    <a href="/sort2" class="nav-link text-white"
                        class:bg-danger=move || location.pathname.get() == "/sort2" >
                        <i class="bi bi-question-lg me-2"></i>
                        Sort2
                    </a>
                </li>
                <li>
                    <a href="/sort3" class="nav-link text-white"
                        class:bg-danger=move || location.pathname.get() == "/sort3" >
                        <i class="bi bi-question-lg me-2"></i>
                        Sort3
                    </a>
                </li>
            </ul>
            <hr/>
                <div class="text-secondary ps-3">
                    <div>Sigtown <i class="bi bi-c-circle mx-2"></i> 2024</div>
                    <a target="_blank" href="https://opensource.org/license/mit/" class="link link-secondary me-1">MIT Licensed</a>
                </div>
        </div>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="container-fluid my-3 text-start p-5">
            <h3>Visual Sorting</h3>
            <p>Sorting algorithms visualized using Rust, Leptos, HTML Canvas, and Bootstrap</p>
        </div>
    }
}

struct Bubble {
    x: usize,
    y: usize,
    data: Vec<usize>,
    done: bool,
    canvas_w: f64,
    canvas_h: f64,
    ctx2d: CanvasRenderingContext2d,
    osc: OscillatorNode,
    gain: GainNode,
    volume: RwSignal<f32>,
}

impl Bubble {
    fn new(
        canvas_ref: &NodeRef<Canvas>,
        items: usize,
        volume: RwSignal<f32>,
        canvas_w: f64,
        canvas_h: f64,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let mut nums: Vec<usize> = (1..=items).collect();
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
            canvas_h,
            canvas_w,
            ctx2d,
            osc: audio_osc,
            gain: audio_gain,
            volume,
        }
    }

    fn draw(&mut self, ticks: usize) {
        self.gain.gain().set_value(self.volume.get_untracked());

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
                // self.ctx2d.set_fill_style(&JsValue::from("#198754")); // bootstrap green
                self.ctx2d.set_fill_style(&JsValue::from("#dddddd")); // bootstrap green
            } else {
                self.ctx2d.set_fill_style(&JsValue::from("#dc3545")); // bootstrap red
            }
            self.ctx2d.begin_path();
            self.ctx2d.rect(x, self.canvas_h - height, width, height);
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
fn BubbleSort(
    play: RwSignal<bool>,
    update_ms: RwSignal<usize>,
    items: RwSignal<usize>,
    volume: RwSignal<f32>,
) -> impl IntoView {
    let mut bubble_holder: Option<Bubble> = None;
    let mut prev_update = 0.0;

    let canvas_w = 1200.0;
    let canvas_h = 600.0;
    let canvas_ref = create_node_ref::<Canvas>();
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
            bubble_holder = Some(Bubble::new(
                &canvas_ref,
                items.get_untracked(),
                volume,
                canvas_w,
                canvas_h,
            ));
        }

        if let Some(bubble) = bubble_holder.as_mut() {
            let now = document.timeline().current_time().unwrap();
            let delta = now - prev_update;
            let ticks = delta as usize / update_ms.get_untracked();
            if ticks > 0 {
                bubble.draw(ticks);
                prev_update = now;
            }

            if !bubble.done && play.get_untracked() {
                let _ = window_clone
                    .request_animation_frame(draw_clone.borrow().as_ref().unchecked_ref());
            } else {
                let _ = bubble.osc.stop();
                bubble_holder = None;
                prev_update = 0.0;
                play.set(false);
            }
        }
    });

    let draw_to_canvas = move |_| {
        play.set(true);
        let _ = window.request_animation_frame(draw.borrow().as_ref().unchecked_ref());
    };

    view! {
        <div class="container-fluid my-3 p-4">
            <div class="d-flex justify-content-start mb-3">
                <button class="col-1 btn btn-outline-danger mx-2"
                    disabled=move || play.get()
                    on:click=draw_to_canvas>
                    <i class="bi bi-play-fill me-2"></i>
                    Play
                </button>
                <button class="col-1 btn mx-2"
                    disabled=move || !play.get()
                    class:btn-outline-warning=move || play.get()
                    class:btn-outline-secondary=move || !play.get()
                    on:click=move |_| play.set(false)>
                    <i class="bi bi-stop-fill me-2"></i>
                    Stop
                </button>
                <span class="d-inline-flex flex-column border rounded p-2 mx-2"
                    class:border-success=move || !play.get()
                    class:border-secondary=move || play.get()>
                    <label class="text-muted me-2">"Items: "{move || items.get()}</label>
                    <input type="range" class="form-range" value=items.get_untracked() min="1" max="200" step="1"
                        disabled=move || play.get()
                        on:input=move |ev| items.set(event_target_value(&ev).parse().unwrap())/>
                </span>
                <span class="d-inline-flex flex-column border border-success rounded p-2 mx-2">
                    <label class="text-muted me-2">"Volume: "{move || (volume.get() * 100.0).floor()}%</label>
                    <input type="range" class="form-range" value=volume.with_untracked(|v| (v*100.0).floor()) min="1" max="100" step="1"
                        on:input=move |ev| volume.set(event_target_value(&ev).parse::<f32>().unwrap() / 100.0)/>
                </span>
                <span class="d-inline-flex flex-column border border-success rounded p-2 mx-2">
                    <label class="text-muted me-2">"Delay "{move || update_ms.get()}"ms"</label>
                    <input type="range" class="form-range w-auto" value=move || update_ms.get() min="1" max="500" step="1"
                        on:input=move |ev| update_ms.set(event_target_value(&ev).parse().expect("to be integer"))/>
                </span>
            </div>
            <div class="d-flex justify-content-start p-2">
                <canvas width=canvas_w height=canvas_h class="col-11 border border-2 rounded border-danger" _ref=canvas_ref />
            </div>
        </div>
    }
}
