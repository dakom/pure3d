extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use web_sys::{WebGlProgram, WebGlRenderingContext};
use super::enums::{DataType};
use super::errors::*;
use super::canvas::*;

#[derive(Clone)]
pub struct WebGlRenderer {
    gl:WebGlRenderingContext
}

impl WebGlRenderer {
    pub fn new(canvas_element: &web_sys::HtmlCanvasElement) -> Result<WebGlRenderer, Error> {
        get_canvas_context_1(&canvas_element)
            .ok_or(Error::from(NativeError::CanvasCreate))
            .map(|gl| WebGlRenderer {gl})
    }

    pub fn context(self:&Self) -> &WebGlRenderingContext {
        &self.gl
    }
}

