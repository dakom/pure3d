extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use web_sys::{console, WebGlBuffer, WebGlRenderingContext, WebGlProgram};
use js_sys::{WebAssembly};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
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
                    start_ticker(gl, &program);
                    on_load.call0(this);
                }
                Err(msg) => {
                    log_string(msg);
                }
            }
        },
        None => {
            on_error.call1(this, &JsValue::from_str("Couldn't get Canvas Context!"));
        }
    };
}

fn start_ticker (gl:WebGlRenderingContext, program:&WebGlProgram) {
    //Upload common data to GPU
    gl.use_program(Some(program));

    let buffer = gl.create_buffer();

    let data:Vec<f32> = vec![  
            0.0,1.0, // top-left
            0.0,0.0, //bottom-left
            1.0, 1.0, // top-right
            1.0, 0.0 // bottom-right
    ];
    

    upload_array_buffer(&gl, &data, WebGlRenderingContext::ARRAY_BUFFER, buffer.as_ref());
    gl.vertex_attrib_pointer_with_i32( 0, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0);

    //Kick off rAF loop
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    {
        //see: https://github.com/rustwasm/wasm-bindgen/blob/master/examples/request-animation-frame/src/lib.rs
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            gl.draw_arrays(WebGlRenderingContext::TRIANGLE_STRIP, 0, 4);
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<FnMut()>));
    }

    request_animation_frame(g.borrow().as_ref().unwrap());

}

// This function is automatically invoked after the wasm module is instantiated.
//
fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn upload_array_buffer(gl:&WebGlRenderingContext, values:&Vec<f32>, target: u32, webgl_buffer:Option<&WebGlBuffer>) -> Result<(), JsValue> {
    wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .map(|m:WebAssembly::Memory| {

            let wasm_buffer = m.buffer();
            let ptr_loc = values.as_ptr() as u32 / 4;

            let float32 = js_sys::Float32Array::new(&wasm_buffer)
                            .subarray(ptr_loc, ptr_loc + values.len() as u32);
    
            gl.bind_buffer(target, webgl_buffer); 

            //Note - WebGL2 can do less GC hits by pointing at same memory with different start/end
            gl.buffer_data_with_array_buffer_view(target, &float32, WebGlRenderingContext::STATIC_DRAW); 
            
        })
}

fn log_string(s:String) {
    log_str(&s[..]);
}
fn log_str(s:&str) {
    console::log_1(&JsValue::from_str(s));
}
