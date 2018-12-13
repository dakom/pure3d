extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub extern "C" fn get_canvas_context(canvas: web_sys::HtmlCanvasElement) -> Option<web_sys::CanvasRenderingContext2d> {
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
