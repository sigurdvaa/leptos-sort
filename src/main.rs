use leptos::html::{Button, Canvas};
use leptos::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};

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

#[component]
fn Canvas() -> impl IntoView {
    let duration = 2000.0;
    let mut start_time = 0.0;
    let mut i = 0.0;

    let canvas_w = 600.0;
    let canvas_h = 350.0;
    let canvas_ref = create_node_ref::<Canvas>();
    let btn_ref = create_node_ref::<Button>();
    let window = web_sys::window().unwrap();
    let window_clone = window.clone();
    let draw: Callback = Rc::new(RefCell::new(Closure::new(move |_| ())));
    let draw_clone = draw.clone();

    *draw.borrow_mut() = Closure::new(move |prev_end_time| {
        if i == 0.0 {
            start_time = prev_end_time;
        }
        i += 1.0;

        let canvas = canvas_ref.get_untracked().expect("canvas should exist");

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        context.clear_rect(0.0, 0.0, canvas_w, canvas_h);
        context.set_fill_style(&JsValue::from("white"));
        context.begin_path();
        context.rect(i * 4.0, 0.0, 100.0, canvas_h);
        context.close_path();
        context.fill();

        let delta = prev_end_time - start_time;
        if delta < duration {
            let _ =
                window_clone.request_animation_frame(draw_clone.borrow().as_ref().unchecked_ref());
        } else {
            i = 0.0;
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
                <canvas class="border border-primary" width=canvas_w height=canvas_h
                ref=canvas_ref />
            </div>
        </div>
    }
}
