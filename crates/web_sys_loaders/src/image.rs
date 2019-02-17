extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use futures::{Future, Async, Poll};
use futures::sync::oneshot::{Sender, Receiver, channel};
use futures::task::current;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;


struct Image {
    url: String,
    img: Option<HtmlImageElement>,
    state: ImageState,
    closure_success:Option<Closure<FnMut()>>,
    closure_err:Option<Closure<FnMut()>>,
    error: Option<JsValue>
}

enum ImageState {
    Empty,
    Loading {
        receiver_err: Receiver<JsValue>
    },
    Ready,
    Error
}
//See: https://github.com/rustwasm/wasm-bindgen/issues/1126
//
impl Future for Image {
    type Item = HtmlImageElement;
    type Error = JsValue;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match &mut self.state {
            ImageState::Empty => {
                let img = HtmlImageElement::new()?;
                let url = self.url.as_str();
                let has_same_origin = same_origin(url)?;
                let (sender_err, receiver_err):(Sender<JsValue>, Receiver<JsValue>) = channel();

                if !has_same_origin {
                    img.set_cross_origin(Some(&"anonymous"));
                }

                img.set_src(url);
                

                //success callback
                let task = current();
                
                let closure_success = Closure::wrap(Box::new(move || {
                    task.notify();
                }) as Box<FnMut()>);
                
                img.set_onload(Some(closure_success.as_ref().unchecked_ref()));
                
                self.closure_success = Some(closure_success);

                //error callback
                let task = current();
                let cb = move || {
                    //TODO - FIX ME!!!
                    //sender_err.send(JsValue::from_str("unknown error!"));
                    //task.notify();
                };

                let closure_err = Closure::wrap(Box::new(cb) as Box<FnMut()>);
                
                img.set_onerror(Some(closure_err.as_ref().unchecked_ref()));
                
                self.closure_err = Some(closure_err);

                //Assign stuff to myself
                self.img = Some(img);
                self.state = ImageState::Loading {receiver_err};

                //notify the task that we're now loading
                let task = current();
                task.notify();

                Ok(Async::NotReady)
            },

            ImageState::Loading {receiver_err} => {
                let err = receiver_err.poll();
                match(err) {
                    Ok(value) => {
                        self.state = ImageState::Error;
                    },
                    _ => {
                        self.state = ImageState::Ready;
                    }
                }

                Ok(Async::NotReady)
            },

            ImageState::Ready => {
                Ok(Async::Ready(self.img.as_ref().unwrap().clone()))
            },

            ImageState::Error => {
                match &self.error {
                    None => Err(JsValue::from_str("unknown error")),
                    Some(err) => Err(err.clone())
                }
            },
        }
    }
}

impl Image {
    fn new(url: String) -> Self {

        Self {
            url,
            img: None,
            state: ImageState::Empty,
            closure_success: None,
            closure_err: None,
            error: None,
        }
    }
}

pub fn fetch_image(url:String) -> impl Future<Item = HtmlImageElement, Error = JsValue> { 
    Image::new(url)

    /*
    let img = HtmlImageElement::new()?;

    let has_same_origin = same_origin(url)?;

    if !has_same_origin {
        img.set_cross_origin(Some(&"anonymous"));
    }

    let cb_onload = |evt:JsValue| {
        //evt is a JsValue... I think... we want to get evt.target
        console::log_1(&evt);
    };

    let cb_onload = Closure::wrap(Box::new(cb_onload) as Box<Fn(JsValue) -> ()>);

    img.set_onload(Some(cb_onload.as_ref().unchecked_ref()));


    let cb_onerror = |evt:JsValue| {
        //evt is a JsValue... I think... we want to get evt.target
        console::error_1(&evt);
    };

    let cb_onerror = Closure::wrap(Box::new(cb_onerror) as Box<Fn(JsValue) -> ()>);

    img.set_onerror(Some(cb_onerror.as_ref().unchecked_ref()));

    //TODO - manage this better... should return in a future somehow?
    cb_onload.forget();
    cb_onerror.forget();

    img.set_src(url);
    Ok(img)
    */
}


pub fn same_origin(url:&str) -> Result<bool, JsValue> {
    //FOLLOWUP: https://github.com/rustwasm/wasm-bindgen/issues/1150
    if url.starts_with("http://") || url.starts_with("https://") {
        let location_origin = get_window()?.location().origin()?; 
        let url_origin = Url::new(url)?.origin();
        Ok(url_origin == location_origin)
    } else {
        Ok(true)
    }
}

fn get_window () -> Result<web_sys::Window, JsValue> {
    web_sys::window().ok_or(JsValue::from_str("couldn't get window"))
}
