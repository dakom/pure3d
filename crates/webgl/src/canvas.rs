extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use wasm_bindgen::JsCast;

/*
 * This whole approach might be better
 * By just passing the web_sys type
 */

pub enum ContextType {
    Canvas2d, //"2d"
    Gl(WebGlVersion), //"webgl" 
}

pub enum WebGlVersion {
    One,
    Two
}

impl ContextType {
    fn to_str(&self) -> &str {

        match self {
            ContextType::Canvas2d => "2d",
            ContextType::Gl(WebGlVersion::One) => "webgl",
            ContextType::Gl(WebGlVersion::Two) => "webgl2",
        }
    }
    pub fn to_string(&self) -> String {
        String::from(self.to_str())
    }
}

//TODO - change the type based on the enum
pub fn get_canvas_context(canvas: web_sys::HtmlCanvasElement, context_type:ContextType) -> Option<web_sys::WebGlRenderingContext> {
    canvas
        .get_context(context_type.to_str())
        .ok()
        .and_then(|object| object)
        .and_then(|object| {
            object
                //.dyn_into::<web_sys::CanvasRenderingContext2d>()
                .dyn_into::<web_sys::WebGlRenderingContext>()
                .ok() 
        })
}
