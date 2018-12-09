extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
//TODO - accept Canvas Element
pub extern fn load_assets(scene_name: &str, on_load: js_sys::Function) {

    log_str(format!("Loading scene: {}", scene_name));
    //TODO - call onLoad
}

fn log_str(s:String) {
    console::log_1(&JsValue::from_str(&s[..]));
}

/* For debugging VIM plugin
#[wasm_bindgen]
pub extern fn foo () {
    log_foo(String::from("foo"));
}

fn log_foo(s:&str) {
}
*/
