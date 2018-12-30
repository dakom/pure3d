mod rust;

extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use crate::rust::scenes::scene::{Scene};
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

    let scene = {
        match scene_name {
            "quad" => QuadScene::new(Rc::clone(&renderer)),
            _ => Err(Error::from("unknown scene!"))
        }
    }?;

    let scene = Rc::new(RefCell::new(*scene));

    start_resize(Rc::clone(&renderer), Rc::clone(&scene))?;
    start_ticker(Rc::clone(&scene))?;

    on_load.call0(this)?;
    Ok(())
}
