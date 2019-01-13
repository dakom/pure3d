use crate::rust::scenes::basic::quad::quad_scene::*;
use crate::rust::scenes::scene::Scene;

//use web_sys::{console};
use crate::rust::helpers::data::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use pure3d_webgl::renderer::*; 
use pure3d_webgl::errors::*; 

pub fn start_resize <T: 'static + Scene + ?Sized>(renderer:Rc<RefCell<WebGlRenderer>>, scene:Rc<RefCell<Box<T>>>) -> Result<(), Error> {
    let cb = move || {
        get_window()
            .and_then(|window| {
                get_window_size(&window)
            })
            .map(|window_size| {
                let mut renderer = renderer.borrow_mut();
                renderer.resize(window_size.width as u32, window_size.height as u32);

                let mut scene = scene.borrow_mut();
                scene.resize(window_size.width as u32, window_size.height as u32);
            })
            .map_err(|err| err.to_js())
    };

    //First we want to resize right away
    cb();

    //Then we need to box it up in a way that can be sent to JS handler
    let cb = Closure::wrap(Box::new(cb) as Box<FnMut() -> Result<(), JsValue>>);

    //And hook it up!
    let window = get_window()?;
    window.set_onresize(Some(cb.as_ref().unchecked_ref()));

    //Purposefully leak memory here so we retain the callback between JS's on_resize handler!
    cb.forget();

    Ok(())
}

pub fn start_ticker <T:'static + Scene + ?Sized>(scene:Rc<RefCell<Box<T>>>) -> Result<(), Error> {
    //Kick off rAF loop
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    {
        //see: https://github.com/rustwasm/wasm-bindgen/blob/master/examples/request-animation-frame/src/lib.rs
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time_stamp| {
            let mut scene = scene.borrow_mut();
            scene.tick(time_stamp);

            //console::log_1(&JsValue::from_str(format!("{}", time_stamp).as_str()));
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
