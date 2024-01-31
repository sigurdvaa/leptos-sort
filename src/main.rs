mod visual_sort;

use leptos::html::Canvas;
use leptos::*;
use leptos_router::*;
use std::cell::RefCell;
use std::rc::Rc;
use visual_sort::{Sort, VisualSort};
use wasm_bindgen::{prelude::Closure, JsCast};

type Callback = Rc<RefCell<Closure<dyn FnMut(f64)>>>;

enum BoostrapColor {
    Green,
    Light,
    Red,
}

impl BoostrapColor {
    fn as_str(&self) -> &str {
        match self {
            Self::Green => "#198754",
            Self::Light => "#dddddd",
            Self::Red => "#dc3545",
        }
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let update_ms = create_rw_signal(25);
    let play = create_rw_signal(false);
    let items = create_rw_signal(50);
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
                        path=Sort::Bubble.route_as_str()
                        view=move || view! { <DisplaySort sort=Sort::Bubble play update_ms items volume/> }
                    />
                    <Route
                        path=Sort::Insertion.route_as_str()
                        view=move || view! { <DisplaySort sort=Sort::Insertion play update_ms items volume/> }
                    />
                    <Route
                        path=Sort::Selection.route_as_str()
                        view=move || view! { <DisplaySort sort=Sort::Selection play update_ms items volume/> }
                    />
                    <Route
                        path=Sort::Quick.route_as_str()
                        view=move || view! { <DisplaySort sort=Sort::Quick play update_ms items volume/> }
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
                    <a href=Sort::Bubble.route_as_str() class="nav-link text-white"
                        class:bg-danger=move || location.pathname.get() == Sort::Bubble.route_as_str() >
                        <i class="bi bi-chat me-2"></i>
                        { Sort::Bubble.name_as_str().to_string() }
                    </a>
                </li>
                <li>
                    <a href=Sort::Insertion.route_as_str() class="nav-link text-white"
                        class:bg-danger=move || location.pathname.get() == Sort::Insertion.route_as_str() >
                        <i class="bi bi-chevron-bar-left me-2"></i>
                        { Sort::Insertion.name_as_str().to_string() }
                    </a>
                </li>
                <li>
                    <a href=Sort::Selection.route_as_str() class="nav-link text-white"
                        class:bg-danger=move || location.pathname.get() == Sort::Selection.route_as_str() >
                        <i class="bi bi-arrows-collapse-vertical me-2"></i>
                        { Sort::Selection.name_as_str().to_string() }
                    </a>
                </li>
                <li>
                    <a href=Sort::Quick.route_as_str() class="nav-link text-white"
                        class:bg-danger=move || location.pathname.get() == Sort::Quick.route_as_str() >
                        <i class="bi bi-vr me-2"></i>
                        { Sort::Quick.name_as_str().to_string() }
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
        <div class="container-fluid my-3 text-start p-4">
            <h3 class="p-2">Visual Sorting</h3>
            <p class="ps-2">
                Sorting algorithms visualized using Rust, Leptos, WASM,
                HTML Canvas, Web Audio API, and Bootstrap
            </p>
        </div>
    }
}

#[component]
fn DisplaySort(
    sort: Sort,
    play: RwSignal<bool>,
    update_ms: RwSignal<usize>,
    items: RwSignal<usize>,
    volume: RwSignal<f32>,
) -> impl IntoView {
    let mut sorter_holder: Option<Box<dyn VisualSort>> = None;
    let mut prev_update = 0.0;

    let access = create_rw_signal(0);
    let swap = create_rw_signal(0);

    let sort_name = sort.name_as_str();
    let canvas_ref = create_node_ref::<Canvas>();
    let window = web_sys::window().unwrap();
    let document = leptos::document();
    let location = use_location();
    let start_loc = location.pathname.get_untracked();

    let draw: Callback = Rc::new(RefCell::new(Closure::new(move |_| ())));
    let draw_clone = draw.clone();

    *draw.borrow_mut() = Closure::new(move |prev_end_time| {
        if prev_update == 0.0 {
            prev_update = prev_end_time;
        }

        if sorter_holder.is_none() {
            access.set(0);
            swap.set(0);
            sorter_holder =
                Some(sort.init(&canvas_ref, items.get_untracked(), volume, access, swap));
        }

        if let Some(bubble) = sorter_holder.as_mut() {
            let now = document.timeline().current_time().unwrap();
            let delta = now - prev_update;
            let ticks = delta as usize / update_ms.get_untracked();
            if ticks > 0 {
                bubble.draw(ticks);
                prev_update = now;
            }

            if !bubble.done()
                && play.get_untracked()
                && start_loc == location.pathname.get_untracked()
            {
                let _ =
                    window.request_animation_frame(draw_clone.borrow().as_ref().unchecked_ref());
            } else {
                bubble.osc_stop();
                sorter_holder = None;
                prev_update = 0.0;
                play.set(false);
            }
        }
    });

    view! {
        <div class="container-fluid my-3 p-4">
            <h3 class="p-2">
                { sort_name.to_string() }
            </h3>
            <Controls play update_ms items volume draw/>
            <div class="d-flex justify-content-start h-75 p-2">
                <canvas class="col-11 border border-1 rounded border-danger" _ref=canvas_ref />
            </div>
            <Details access swap/>
        </div>
    }
}

#[component]
fn Details(access: RwSignal<usize>, swap: RwSignal<usize>) -> impl IntoView {
    view! {
        <div class="ps-2">"Array access: "{move || access.get()}</div>
        <div class="ps-2">"Array swap: "{move || swap.get()}</div>
    }
}

#[component]
fn Controls(
    play: RwSignal<bool>,
    update_ms: RwSignal<usize>,
    items: RwSignal<usize>,
    volume: RwSignal<f32>,
    draw: Callback,
) -> impl IntoView {
    let window = web_sys::window().expect("window should exists");
    let draw_to_canvas = move |_| {
        play.set(true);
        let _ = window.request_animation_frame(draw.borrow().as_ref().unchecked_ref());
    };

    view! {
        <div class="d-flex justify-content-start mb-3">
            // play
            <button class="col-1 btn btn-outline-danger mx-2"
                disabled=move || play.get()
                on:click=draw_to_canvas>
                <i class="bi bi-play-fill me-2"></i>
                Play
            </button>
            // stop
            <button class="col-1 btn mx-2"
                disabled=move || !play.get()
                class:btn-outline-warning=move || play.get()
                class:btn-outline-secondary=move || !play.get()
                on:click=move |_| play.set(false)>
                <i class="bi bi-stop-fill me-2"></i>
                Stop
            </button>
            // items
            <span class="d-inline-flex flex-column border rounded p-2 mx-2"
                class:border-success=move || !play.get()
                class:border-secondary=move || play.get()>
                <label class="text-muted me-2">"Items: "{move || items.get()}</label>
                <input type="range" class="form-range" min="1" max="300" step="1"
                    disabled=move || play.get()
                    prop:value=items.get_untracked()
                    on:input=move |ev| items.set(event_target_value(&ev).parse().expect("integer"))/>
            </span>
            // volume
            <span class="d-inline-flex flex-column border border-success rounded p-2 mx-2">
                <label class="text-muted me-2">"Volume: "{move || (volume.get() * 100.0).floor()}%</label>
                <input type="range" class="form-range" min="0" max="100" step="1"
                    prop:value=volume.with_untracked(|v| (v * 100.0).floor())
                    on:input=move |ev| volume.set(event_target_value(&ev).parse::<f32>().expect("f32") / 100.0)/>
            </span>
            // update ms
            <span class="d-inline-flex flex-column border border-success rounded p-2 mx-2">
                <label class="text-muted me-2">"Delay "{move || update_ms.get()}"ms"</label>
                <input type="range" class="form-range w-auto" min="1" max="500" step="1"
                    prop:value=update_ms.get_untracked()
                    on:input=move |ev| update_ms.set(event_target_value(&ev).parse().expect("integer"))/>
            </span>
        </div>
    }
}
