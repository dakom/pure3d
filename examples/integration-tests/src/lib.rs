mod rust;

extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;

use crate::rust::scenes::scene::{Scene};
use crate::rust::scenes::basic::quad::quad_scene::*;
use crate::rust::scenes::basic::quad_texture::quad_texture_scene::*;
use crate::rust::dom_handlers::*;
use wasm_bindgen::prelude::*;
use pure3d_webgl::renderer::*; 
use pure3d_webgl::errors::*; 
use std::rc::Rc;
use std::cell::RefCell;
use futures::future::{Future, result};
use wasm_bindgen_futures::{JsFuture, future_to_promise};

#[wasm_bindgen]
pub extern "C" fn run(
    canvas: web_sys::HtmlCanvasElement, 
    scene_name: String, 
) -> js_sys::Promise {

    let this = &JsValue::NULL;

    future_to_promise(
        //Yeah this is getting nasty...
        //Should look MUCH nicer if we can use async/await
        result(WebGlRenderer::new(canvas))
            .and_then(move |webgl_renderer| {
                let webgl_renderer = Rc::new(RefCell::new(webgl_renderer));
                get_scene(scene_name, Rc::clone(&webgl_renderer))
                    .map(|scene| Rc::new(RefCell::new(scene)))
                    .and_then(move |scene| {
                        result(
                            start_ticker(Rc::clone(&scene))
                                .and_then(|_| {
                                    start_resize(Rc::clone(&webgl_renderer), Rc::clone(&scene))
                                })
                        )
                    })
            })
            .map_err(|err| err.to_js())
            .map(|_| { JsValue::from_str("it works!") })
    )
}

//should work... see: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=adfb0e3366e47fa59a4942a48376f685
fn get_scene(scene_name:String, webgl_renderer:Rc<RefCell<WebGlRenderer>>) -> Box<dyn Future<Item = Box<dyn Scene + 'static>, Error = Error>>{
    let scene_name = scene_name.as_str();

    match(scene_name) {
        "quad" => Box::new(QuadScene::new(webgl_renderer).map(|scene| scene as Box<Scene + 'static>)),
        "quad_texture" => Box::new(QuadTextureScene::new(webgl_renderer).map(|scene| scene as Box<Scene + 'static>)),
        _ => Box::new(futures::future::err(Error::from("unknown scene!")))
    }
}
