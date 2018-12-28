use crate::rust::scenes::basic::quad::quad_scene::*;
use crate::rust::scenes::scene::Scene;

use web_sys::{console};
use crate::rust::helpers::data::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use pure3d_webgl::renderer::*; 
use pure3d_webgl::errors::*; 

pub fn start_resize(renderer:Rc<RefCell<WebGlRenderer>>) -> Result<(), Error> {
    let window = get_window()?;
    let window_size = get_window_size(&window)?;

    let mut renderer = renderer.borrow_mut();
    renderer.resize(window_size.width as u32, window_size.height as u32);

    let cb = Closure::wrap(Box::new(move |width, height| {
        let s = format!("got resize! {} {}", width, height);
        console::log_1(&JsValue::from_str(s.as_str()));
    }) as Box<FnMut(u32, u32) -> ()>);

    //window.set_onresize(Some(&js_sys::Function::from(cb)));

    Ok(())
}

pub fn start_raf <T:'static + Scene>(mut scene:T, renderer:Rc<RefCell<WebGlRenderer>>) -> Result<(), JsValue> {


    //Kick off rAF loop
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    {
        //see: https://github.com/rustwasm/wasm-bindgen/blob/master/examples/request-animation-frame/src/lib.rs
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time_stamp| {

            let mut renderer = renderer.borrow_mut();

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

fn get_window () -> Result<web_sys::Window, Error> {
    web_sys::window().ok_or(Error::from("couldn't get window"))
}
