extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use web_sys::{console, WebGlBuffer, WebGlRenderingContext, WebGlProgram};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use pure3d_webgl::*; 
use pure3d_webgl::errors::*;
use pure3d_webgl::enums::*;

mod helpers;
use crate::helpers::*;

#[wasm_bindgen]
pub extern "C" fn load_assets(
    canvas_element: web_sys::HtmlCanvasElement, 
    scene_name: &str, 
    on_load: js_sys::Function,
    on_error: js_sys::Function,
) -> Result<(), JsValue> {

    let this = &JsValue::NULL;
  
    let result = create_context(canvas_element)
        .and_then(|gl| 
            shader::compile_shader(&gl, 
                include_str!("shaders/Quad-Vertex.glsl"),
                include_str!("shaders/Quad-Fragment.glsl")
            )
            .map(|program| (gl, program))
        )
        .and_then(|(gl, program)| {

            upload_data(&gl, &program)
                .map(|buffer| (buffer, gl, program))
        })
        .and_then(|(buffer, gl, program)| {
            gl.use_program(Some(&program));
            attributes::get_attribute_location(&gl, &program, "a_vertex")
                .ok_or(Error::from(NativeError::AttributeLocation))
                .map(|loc| {
                    let opts = attributes::AttributeOptions::new(2, DataType::Float);
                    attributes::activate_attribute(&gl, &loc, &opts);

                    (gl, program)
                })
        })
        .and_then(|(gl, program)| {
            gl.enable(WebGlRenderingContext::BLEND);
            let pos = Point{x: 200.0, y: 200.0};
            let area = Area{width: 10.0, height: 100.0};
            let col = Color{r:1.0, g: 1.0, b: 0.0, a: 1.0};
            change_quad(&gl, &program, &pos, &area, &col);

            Ok((gl, program))
        })
        .and_then(|(gl, program)| {
            start_ticker(gl, program)?;
            on_load.call0(this).map_err(|err| Error::from(err))
        });

    match result {
        Err(err) => {
            on_error.call1(this, &err.to_js())?;
        }
        _ => {} 
    }
    Ok(())
}

fn change_quad(gl:&WebGlRenderingContext, program:&WebGlProgram, pos:&Point, area:&Area, color:&Color) {
    gl.get_uniform_location(&program, "u_color")
        .map(|loc| {
            let mut values = color.as_slice();
            gl.uniform4fv_with_f32_array(Some(&loc), &mut values)
        });
}

fn create_context (canvas_element: web_sys::HtmlCanvasElement) -> Result<WebGlRenderingContext, Error> {
    canvas::get_canvas_context(canvas_element, canvas::ContextType::Gl(canvas::WebGlVersion::One))
        .ok_or(Error::from(NativeError::CanvasCreate))
}

fn upload_data(gl:&WebGlRenderingContext, program:&WebGlProgram) -> Result<WebGlBuffer, Error> {
    gl.create_buffer()
        .map_or(Err(Error::from("Couldn't create buffer")), |buffer| {
            let data:Vec<f32> = vec![  
                    0.0,1.0, // top-left
                    0.0,0.0, //bottom-left
                    1.0, 1.0, // top-right
                    1.0, 0.0 // bottom-right
            ];

            buffer::upload_array_buffer(&gl, &data, &BufferTarget::ArrayBuffer, &BufferUsage::StaticDraw, &buffer)
                .map(move |_| buffer)
        })
}

fn start_ticker (gl:WebGlRenderingContext, program:WebGlProgram) -> Result<(), JsValue> {

    //just for fun!
    let pos = Point{x: 200.0, y: 200.0};
    let area = Area{width: 10.0, height: 100.0};
    let mut color = Color{r:1.0, g: 1.0, b: 0.0, a: 1.0};
    let mut direction = 0.01;
    //Kick off rAF loop
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    {
        //see: https://github.com/rustwasm/wasm-bindgen/blob/master/examples/request-animation-frame/src/lib.rs
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            //gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
            color.r += direction;
            if(direction > 0.0) {
                if(color.r > 1.0) {
                    direction *= -1.0;
                    color.r = 1.0;
                }
            } else {
                if(color.r < 0.0) {
                    direction *= -1.0;
                    color.r = 0.0;
                }
            }

            change_quad(&gl, &program, &pos, &area, &color);
            gl.draw_arrays(WebGlRenderingContext::TRIANGLE_STRIP, 0, 4);
            request_animation_frame(f.borrow().as_ref().unwrap())
                .ok()
                .unwrap();
        }) as Box<FnMut()>));
    }

    request_animation_frame(g.borrow().as_ref().unwrap())?;
    Ok(())
}


// This function is automatically invoked after the wasm module is instantiated.
//
fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<FnMut()>) -> Result<(), JsValue> {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref()).and(Ok(()))
}

