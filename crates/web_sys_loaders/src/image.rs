extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use futures::{Future, Async, Poll};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;


struct Image {
    url: String,
    img: Option<HtmlImageElement>,
    state: ImageState
}

enum ImageState {
    Empty,
    Loading,
    Ready
}

impl Future for Image {
    type Item = HtmlImageElement;
    type Error = JsValue;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match &self.state {
            ImageState::Empty => {
                console::log_1(&JsValue::from_str("empty"));
                /*
                let img = HtmlImageElement::new()?;
                let url = self.url.as_str();
                let has_same_origin = same_origin(url)?;

                if !has_same_origin {
                    img.set_cross_origin(Some(&"anonymous"));
                }

                img.set_src(url);
                self.img = Some(img);
                */
                self.state = ImageState::Loading;
                Ok(Async::NotReady)
            },
            ImageState::Loading => {
                console::log_1(&JsValue::from_str("loading"));
                self.state = ImageState::Ready;
                Ok(Async::NotReady)
            },
            ImageState::Ready => {
                console::log_1(&JsValue::from_str("ready"));
                Ok(Async::Ready(self.img.as_ref().unwrap().clone()))
            }
        }
    }
}

impl Image {
    fn new(url: String) -> Image {
        Image {
            url,
            img: None,
            state: ImageState::Empty
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
