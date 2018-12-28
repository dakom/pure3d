mod rust;

extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use crate::rust::scenes::basic::quad::quad_scene::*;
use crate::rust::dom_handlers::*;
use wasm_bindgen::prelude::*;
use pure3d_webgl::renderer::*; 
use pure3d_webgl::errors::*; 
use std::rc::Rc;
use std::cell::RefCell;
#[wasm_bindgen]
pub extern "C" fn run(
    canvas: web_sys::HtmlCanvasElement, 
    scene_name: &str, 
    on_load: js_sys::Function,
) -> Result<(), JsValue> {

    let this = &JsValue::NULL;
    let renderer = WebGlRenderer::new(canvas)?;
    let renderer = Rc::new(RefCell::new(renderer));

    start_resize(Rc::clone(&renderer))?;

    let scene = {
        match scene_name {
            "quad" => QuadScene::new(&mut *renderer.borrow_mut()),
            _ => Err(Error::from("unknown scene!"))
        }
    }?;

    start_raf(scene, Rc::clone(&renderer))?;

    on_load.call0(this)?;

    Ok(())
}
