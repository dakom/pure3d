mod rust;

extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use crate::rust::scenes::basic::quad::quad_scene::*;
use crate::rust::scenes::scene::Scene;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use pure3d_webgl::renderer::*; 
use pure3d_webgl::errors::*; 

#[wasm_bindgen]
pub extern "C" fn run(
    canvas_element: web_sys::HtmlCanvasElement, 
    scene_name: &str, 
    on_load: js_sys::Function,
) -> Result<(), JsValue> {

    let this = &JsValue::NULL;
    let renderer = WebGlRenderer::new(&canvas_element)?;
   
    let scene = {
        match scene_name {
            "quad" => QuadScene::new(&renderer),
            _ => Err(Error::from("unknown scene!"))
        }
    }?;

    start_ticker(scene)?;
    on_load.call0(this)?;

    Ok(())
}

fn start_ticker <T:'static + Scene>(mut scene:T) -> Result<(), JsValue> {

    //Kick off rAF loop
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    {
        //see: https://github.com/rustwasm/wasm-bindgen/blob/master/examples/request-animation-frame/src/lib.rs
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time_stamp| {
            scene.update(time_stamp);
            scene.render();

            if scene.should_stop() {
                f.borrow_mut().take();
            } else {
                request_animation_frame(f.borrow().as_ref().unwrap())
                    .ok()
                    .unwrap();
            }
        }) as Box<FnMut(f64)-> ()>));
    }

    request_animation_frame(g.borrow().as_ref().unwrap())?;
    Ok(())
}

fn request_animation_frame(f: &Closure<FnMut(f64) -> ()>) -> Result<i32, JsValue> {
    let window = web_sys::window().ok_or(Error::from("couldn't get window"))?;
    window.request_animation_frame(f.as_ref().unchecked_ref())
}

