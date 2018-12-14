extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use web_sys::console;
use std::f64;
use pure3d_webgl::*; 

#[wasm_bindgen]
pub extern "C" fn load_assets(
    canvas_element: web_sys::HtmlCanvasElement, 
    scene_name: &str, 
    on_load: js_sys::Function,
    on_error: js_sys::Function,
) {

    let this = &JsValue::NULL;

    let vertex_shader_source  = include_str!("shaders/Quad-Vertex.glsl");
    let fragment_shader_source  = include_str!("shaders/Quad-Fragment.glsl");

    match canvas::get_canvas_context(canvas_element, canvas::ContextType::Gl(canvas::WebGlVersion::One)) {
        Some(gl) => {
            let result = shader::compile_shader(&gl, vertex_shader_source, fragment_shader_source);
            match result {
                Ok(program) => log_str("Got program!!!"),
                Err(msg) => log_str(msg)
            }
            //on_load.call0(this);
        },
        None => {
            on_error.call1(this, &JsValue::from_str("Couldn't get Canvas Context!"));
        }
    };
}

fn draw_quad(context: web_sys::WebGlRenderingContext) {
}

fn log_string(s:String) {
    log_str(&s[..]);
}
fn log_str(s:&str) {
    console::log_1(&JsValue::from_str(s));
}
