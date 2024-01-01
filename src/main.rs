use leptos::html::Canvas;
use leptos::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{prelude::Closure, Clamped, JsCast, JsValue};
use web_sys::ImageData;

type Callback = Rc<RefCell<Closure<dyn FnMut(f64)>>>;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <CanvasDraw/>
        <CanvasBitmap/>
    }
}

#[component]
fn CanvasDraw() -> impl IntoView {
    let duration = 5000.0;
    let mut start_time = 0.0;
    let mut i = 0.0;

    let canvas_ref = create_node_ref::<Canvas>();
    let window = web_sys::window().unwrap();
    let window_clone = window.clone();
    let draw: Callback = Rc::new(RefCell::new(Closure::new(move |_| ())));
    let draw_clone = draw.clone();
    let perf = window.performance().unwrap();
    let mut total_frame_time = 0.0;

    // let mut setup = false;

    *draw.borrow_mut() = Closure::new(move |prev_end_time| {
        let frame_start = perf.now();

        if i == 0.0 {
            start_time = prev_end_time;
        }
        i += 1.0;

        let canvas = canvas_ref.get_untracked().expect("canvas should exist");

        canvas.set_width(1600);
        canvas.set_height(800);

        let canvas_w = canvas.width() as f64;
        let canvas_h = canvas.height() as f64;
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        // if !setup {
        // Set color
        // context.set_stroke_style(&JsValue::from("white"));
        context.set_fill_style(&JsValue::from("white"));
        // context.set_font("26px sans-serif");
        // setup = true;
        // }

        context.clear_rect(0.0, 0.0, canvas_w, canvas_h);
        context.begin_path();
        context.rect(i, 0.0, 50.0, canvas_h);
        context.close_path();
        context.fill();

        let delta = prev_end_time - start_time;
        if delta < duration {
            let fps = i / delta * 1000.0;
            let frame_delta = perf.now() - frame_start;
            total_frame_time += frame_delta;
            logging::log!("Iter: {i}\n  Time: {delta}\n  FPS: {fps}\n  Frame delta: {frame_delta}");
            let _ =
                window_clone.request_animation_frame(draw_clone.borrow().as_ref().unchecked_ref());
        } else {
            logging::log!("avg. frame time: {}", total_frame_time / i);
            i = 0.0;
        }
    });

    let draw_to_canvas = move |_| {
        let _ = window.request_animation_frame(draw.borrow().as_ref().unchecked_ref());
    };

    view! {
        <div class="container-fluid my-3">
            <div class="d-flex justify-content-center mb-3">
                <button class="col-2 btn btn-primary" on:click=draw_to_canvas>
                    CanvasDraw
                </button>
            </div>
            <div class="d-flex justify-content-center mb-3">
                <canvas class="border border-primary" ref=canvas_ref />
            </div>
        </div>
    }
}

fn draw_rect(
    data: &mut [u8],
    width: usize,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
) {
    let mut px;
    for x in start_x..end_x {
        for y in start_y..end_y {
            px = (x + (y * width)) * 4;
            data[px] = 255;
            data[px + 1] = 255;
            data[px + 2] = 255;
            data[px + 3] = 255;
        }
    }
}

#[component]
fn CanvasBitmap() -> impl IntoView {
    let duration = 5000.0;
    let mut start_time = 0.0;
    let mut i = 0;

    let canvas_ref = create_node_ref::<Canvas>();
    let window = web_sys::window().unwrap();
    let window_clone = window.clone();
    let draw: Callback = Rc::new(RefCell::new(Closure::new(move |_| ())));
    let draw_clone = draw.clone();
    let perf = window.performance().unwrap();
    let mut total_frame_time = 0.0;

    *draw.borrow_mut() = Closure::new(move |timestamp| {
        let frame_start = perf.now();

        if i == 0 {
            start_time = timestamp;
        }
        i += 1;

        let canvas = canvas_ref.get_untracked().expect("canvas should exist");

        canvas.set_width(1600);
        canvas.set_height(800);

        let canvas_w = canvas.width() as f64;
        let canvas_h = canvas.height() as f64;
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let mut data = vec![0; canvas_w as usize * canvas_h as usize * 4];
        let x = i as usize;
        draw_rect(
            &mut data,
            canvas_w as usize,
            x,
            0,
            x + 50,
            canvas_h as usize,
        );

        let clamped = Clamped(&data[..]);
        let image =
            ImageData::new_with_u8_clamped_array_and_sh(clamped, canvas_w as u32, canvas_h as u32)
                .expect("imagedata created");
        let _ = context.put_image_data(&image, 0.0, 0.0);

        let delta = timestamp - start_time;
        if delta < duration {
            let fps = i as f64 / delta * 1000.0;
            let frame_delta = perf.now() - frame_start;
            total_frame_time += frame_delta;
            logging::log!("Iter: {i}\n  Time: {delta}\n  FPS: {fps}\n  frame delta: {frame_delta}");
            let _ =
                window_clone.request_animation_frame(draw_clone.borrow().as_ref().unchecked_ref());
        } else {
            logging::log!("avg. frame time: {}", total_frame_time / i as f64);
            i = 0;
            data.clear();
        }
    });

    let draw_to_canvas = move |_| {
        let _ = window.request_animation_frame(draw.borrow().as_ref().unchecked_ref());
    };

    view! {
        <div class="container-fluid my-3">
            <div class="d-flex justify-content-center mb-3">
                <button class="col-2 btn btn-primary" on:click=draw_to_canvas>
                    CanvasBitmap
                </button>
            </div>
            <div class="d-flex justify-content-center mb-3">
                <canvas class="border border-primary" ref=canvas_ref />
            </div>
        </div>
    }
}
