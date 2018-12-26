mod quad;
mod helpers;

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
use pure3d_webgl::renderer::*; 
use crate::quad::{Quad, QuadRenderer};

#[wasm_bindgen]
pub extern "C" fn run(
    canvas_element: web_sys::HtmlCanvasElement, 
    scene_name: &str, 
    on_load: js_sys::Function,
) -> Result<(), JsValue> {

    let this = &JsValue::NULL;
    let renderer = WebGlRenderer::new(&canvas_element)?;
    let quad = Quad::new();
    let quad_renderer = QuadRenderer::new(renderer.clone())?;
    start_ticker(quad, quad_renderer)?;
    on_load.call0(this)?;

    Ok(())
}

fn start_ticker (mut quad:Quad, quad_renderer:QuadRenderer) -> Result<(), JsValue> {
    //just for fun!
    let mut direction = 0.01;
    //Kick off rAF loop
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    {
        //see: https://github.com/rustwasm/wasm-bindgen/blob/master/examples/request-animation-frame/src/lib.rs
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {

            //gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
            direction = quad.update(direction);
            quad_renderer.render(&mut quad);
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

