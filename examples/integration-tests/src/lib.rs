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

//TODO: related to the error above... dyn Scene + 'static feels odd... maybe there's a better
//signature...
fn get_scene(scene_name:String, webgl_renderer:Rc<RefCell<WebGlRenderer>>) -> impl Future<Item = Rc<RefCell<Box<dyn Scene + 'static>>>, Error = Error>{

    let scene_name = scene_name.as_str();

    //Uncommenting either path (happy or sad) works - but when there's both, it's a compile error
    let scene = match scene_name {
        _ => QuadTextureScene::new(webgl_renderer).map(|scene| scene as Box<dyn Scene>),
        //_ => QuadScene::new(webgl_renderer).map(|scene| Rc::new(RefCell::new(scene as Box<dyn Scene>))),
        //"quad" => QuadScene::new(webgl_renderer).map(|scene| Rc::new(RefCell::new(scene as Box<dyn Scene>))),
        //_ => result(Err(Error::from(format!("unknown scene! {}", scene_name))))
    };
    scene.map(|s| {
        Rc::new(RefCell::new(s))
    })
   /* 
    let scene = match scene_name {
        "quad" => QuadScene::new(webgl_renderer).map(|scene| scene as Box<dyn Scene>),
        _ => QuadScene::new(webgl_renderer).map(|scene| scene as Box<dyn Scene>)
        //"quad" => QuadScene::new(webgl_renderer).map(|scene| scene as Box<dyn Scene>),
        //"quad_texture" => QuadTextureScene::new(webgl_renderer).map(|scene| scene as Box<dyn Scene>),
        //_ => result(Err(Error::from(format!("unknown scene! {}", scene_name))))
    };

    */
}
