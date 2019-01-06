mod rust;

extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use crate::rust::scenes::scene::{Scene};
use crate::rust::scenes::basic::quad::quad_scene::*;
use crate::rust::scenes::basic::quad_texture::quad_texture_scene::*;
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
    let webgl_renderer = WebGlRenderer::new(canvas)?;
    let webgl_renderer = Rc::new(RefCell::new(webgl_renderer));
    let scene = get_scene(scene_name, Rc::clone(&webgl_renderer))?;        

    start_resize(Rc::clone(&webgl_renderer), Rc::clone(&scene))?;
    start_ticker(Rc::clone(&scene))?;

    on_load.call0(this)?;
    Ok(())
}

fn get_scene(scene_name:&str, webgl_renderer:Rc<RefCell<WebGlRenderer>>) -> Result<Rc<RefCell<Box<dyn Scene>>>, Error>{
    let scene = match scene_name {
        "quad" => QuadScene::new(webgl_renderer).map(|scene| scene as Box<dyn Scene>),
        "quad_texture" => QuadTextureScene::new(webgl_renderer).map(|scene| scene as Box<dyn Scene>),
        _ => Err(Error::from("unknown scene!"))
    }?;

    Ok(Rc::new(RefCell::new(scene)))
}
