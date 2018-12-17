extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use web_sys::{console, WebGlRenderingContext, WebGlProgram, WebGlShader};
use js_sys::{Float32Array, WebAssembly};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
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

    match canvas::get_canvas_context(canvas_element, canvas::ContextType::Gl(canvas::WebGlVersion::One)) {
        Some(gl) => {
            let result = shader::compile_shader(&gl, 
                include_str!("shaders/Quad-Vertex.glsl"),
                include_str!("shaders/Quad-Fragment.glsl")
            );

            match result {
                Ok(program) => {
                    start_demo(&gl, &program);
                    start_demo(&gl, &program);
                }
                Err(msg) => log_string(msg)
            }
            //on_load.call0(this);
        },
        None => {
            on_error.call1(this, &JsValue::from_str("Couldn't get Canvas Context!"));
        }
    };
}

fn start_demo(gl:&WebGlRenderingContext, program:&WebGlProgram) {
    let data = create_array_buffer(&gl, vec![  
            0.0,1.0, // top-left
            0.0,0.0, //bottom-left
            1.0, 1.0, // top-right
            1.0, 0.0 // bottom-right
    ]).unwrap();
    let buffer = gl.create_buffer();
    gl.use_program(Some(program));
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, buffer.as_ref()); 
    gl.buffer_data_with_opt_array_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&data.buffer()), WebGlRenderingContext::STATIC_DRAW); 

    gl.vertex_attrib_pointer_with_i32(
        0,
        2,
        WebGlRenderingContext::FLOAT,
        false,
        0,
        0
    );

    gl.enable_vertex_attrib_array(0);
  
    gl.draw_arrays(WebGlRenderingContext::TRIANGLE_STRIP, 0, 4);

}

fn create_array_buffer(gl:&WebGlRenderingContext, values:Vec<f32>) -> Result<Float32Array, JsValue> {
    wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .map(|m:WebAssembly::Memory| {
            let buffer = m.buffer();
            let ptr_loc = values.as_ptr() as u32 / 4;

            js_sys::Float32Array::new(&buffer)
                .subarray(ptr_loc, ptr_loc + values.len() as u32)
        })
}

fn draw_quad(context: web_sys::WebGlRenderingContext) {
}

fn log_string(s:String) {
    log_str(&s[..]);
}
fn log_str(s:&str) {
    console::log_1(&JsValue::from_str(s));
}
