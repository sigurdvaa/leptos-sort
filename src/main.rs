use leptos::wasm_bindgen::JsCast;
use leptos::*;
use wasm_bindgen::JsValue;

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

#[component]
fn Canvas() -> impl IntoView {
    let canvas_ref = create_node_ref::<leptos::html::Canvas>();

    let draw_to_canvas = move |_| {
        let canvas = canvas_ref.get().expect("canvas should be created");

        canvas.set_width(600);
        canvas.set_height(600);

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        context.begin_path();

        // Set color
        context.set_stroke_style(&JsValue::from("white"));

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

        context.stroke();
    };

    view! {
        <div class="container-fluid text-center">
            <button class="btn btn-primary" on:click=draw_to_canvas>Draw</button>
            <canvas class="d-inline" _ref=canvas_ref />
        </div>
    }
}
