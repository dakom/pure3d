mod rust;

extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use crate::rust::scenes::basic::quad::quad_scene::*;
use crate::rust::scenes::scene::Scene;

use crate::rust::helpers::data::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use pure3d_webgl::renderer::*; 
use pure3d_webgl::errors::*; 
use std::mem;

#[wasm_bindgen]
pub extern "C" fn run(
    canvas: web_sys::HtmlCanvasElement, 
    scene_name: &str, 
    on_load: js_sys::Function,
) -> Result<(), JsValue> {

    let this = &JsValue::NULL;
    let mut renderer = WebGlRenderer::new(canvas)?;

    let window = get_window()?;
    let window_size = get_window_size(&window)?;

    renderer.resize(window_size.width as u32, window_size.height as u32);
    
    let scene = {
        match scene_name {
            "quad" => QuadScene::new(&renderer, &window_size),
            _ => Err(Error::from("unknown scene!"))
        }
    }?;

    start_ticker(scene, renderer)?;
    //start_resize(renderer); //??? Probably need to use Rc...

    on_load.call0(this)?;

    Ok(())
}

fn start_resize (mut renderer:WebGlRenderer) {
}
fn start_ticker <T:'static + Scene>(mut scene:T, mut renderer:WebGlRenderer) -> Result<(), JsValue> {

    //Kick off rAF loop
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    {
        //see: https://github.com/rustwasm/wasm-bindgen/blob/master/examples/request-animation-frame/src/lib.rs
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time_stamp| {
            scene.update_data(time_stamp);
            scene.update_renderer(&mut renderer);
            scene.render(&renderer);

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

fn get_window () -> Result<web_sys::Window, Error> {
    web_sys::window().ok_or(Error::from("couldn't get window"))
}

fn request_animation_frame(f: &Closure<FnMut(f64) -> ()>) -> Result<i32, JsValue> {
    let window = get_window()?;
    window.request_animation_frame(f.as_ref().unchecked_ref())
}

fn get_window_size(window:&web_sys::Window) -> Result<Area, Error> {
    let inner_width = window.inner_width()?;
    let inner_width = inner_width.as_f64().ok_or(Error::from("couldn't get window width"))?;

    let inner_height= window.inner_height()?;
    let inner_height= inner_height.as_f64().ok_or(Error::from("couldn't get window height"))?;

    Ok(Area{width: inner_width, height: inner_height})
}
