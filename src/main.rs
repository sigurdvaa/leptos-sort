use leptos::html::Canvas;
use leptos::wasm_bindgen::JsCast;
use leptos::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{prelude::Closure, JsValue};

type Callback = Rc<RefCell<Closure<dyn FnMut(f64)>>>;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Canvas/>
    }
}

fn bubblesort(list: &mut [usize]) {
    for i in 0..list.len() {
        for j in 0..list.len() - i - 1 {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
            }
        }
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
    let duration = 3000.0;
    let mut start_time = 0.0;
    let mut i = 0.0;

    let canvas_ref = create_node_ref::<Canvas>();
    let window = web_sys::window().unwrap();
    let window_clone = window.clone();
    let draw: Callback = Rc::new(RefCell::new(Closure::new(move |_| ())));
    let draw_clone = draw.clone();

    let mut setup = false;

    *draw.borrow_mut() = Closure::new(move |timestamp| {
        if i == 0.0 {
            start_time = timestamp;
        }
        i += 1.0;

        let canvas = canvas_ref.get_untracked().expect("canvas should exist");

        // canvas.set_width(600);
        // canvas.set_height(600);

        let canvas_w = canvas.width() as f64;
        let canvas_h = canvas.height() as f64;
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        if !setup {
            // Set color
            context.set_stroke_style(&JsValue::from("white"));
            context.set_fill_style(&JsValue::from("white"));
            context.set_font("26px sans-serif");
            setup = true;
        }

        // Clear
        context.clear_rect(0.0, 0.0, canvas_w, canvas_h);

        // Write text
        let _ = context.fill_text(&format!("Frame: {i}"), 0.0, 150.0);

        context.begin_path();

        // Draw the outer circle.
        context
            .arc(75.0, 75.0, 50.0, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();

        // Draw the mouth.
        context.move_to(110.0, 75.0);
        context
            .arc(75.0, 75.0, 35.0, 0.0, std::f64::consts::PI)
            .unwrap();

        // Draw the left eye.
        context.move_to(65.0, 65.0);
        context
            .arc(60.0, 65.0, 5.0, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();

        // Draw the right eye.
        context.move_to(95.0, 65.0);
        context
            .arc(90.0, 65.0, 5.0, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();

        context.close_path();

        context.stroke();

        let delta = timestamp - start_time;
        if delta < duration {
            let fps = i / delta * 1000.0;
            logging::log!("Iter: {i}\n  Time: {delta}\n  FPS: {fps}");
            let _ =
                window_clone.request_animation_frame(draw_clone.borrow().as_ref().unchecked_ref());
        } else {
            i = 0.0;
        }
    });

    let draw_to_canvas = move |_| {
        let _ = window.request_animation_frame(draw.borrow().as_ref().unchecked_ref());
    };

    view! {
        <div class="container-fluid my-3">
            <div class="d-flex justify-content-center mb-3">
                <button class="col-1 btn btn-primary" on:click=draw_to_canvas>Draw</button>
            </div>
            <div class="d-flex justify-content-center mb-3">
                <canvas ref=canvas_ref />
            </div>
        </div>
    }
}
