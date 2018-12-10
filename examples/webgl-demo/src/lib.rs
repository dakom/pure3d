extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::console;
use wasm_bindgen::JsCast;
use std::f64;


#[wasm_bindgen]
pub extern fn load_assets(
    canvas: web_sys::HtmlCanvasElement, 
    scene_name: &str, 
    on_load: js_sys::Function,
    on_error: js_sys::Function,
) {

    let this = &JsValue::NULL;

    match get_canvas_context(canvas) {
        Some(foo) => {
            draw_happy_face(foo);
            on_load.call0(this);
        },
        None => {
            on_error.call1(this, &JsValue::from_str("Couldn't get Canvas Context!"));
        }
    };
}

fn get_canvas_context(canvas: web_sys::HtmlCanvasElement) -> Option<web_sys::CanvasRenderingContext2d> {
    canvas
        .get_context("2d")
        .ok()
        .and_then(|object| object)
        .and_then(|object| {
            object
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .ok() 
        })
}

fn draw_happy_face(context: web_sys::CanvasRenderingContext2d) {

    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();
}

fn log_str(s:String) {
    console::log_1(&JsValue::from_str(&s[..]));
}
