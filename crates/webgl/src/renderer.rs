extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use web_sys::{HtmlCanvasElement, WebGlProgram, WebGlRenderingContext};
use super::enums::{DataType};
use super::errors::*;
use super::canvas::*;

pub struct WebGlRenderer {
    gl:WebGlRenderingContext,
    canvas: HtmlCanvasElement,
    last_width: u32,
    last_height: u32
}

impl WebGlRenderer {
    pub fn new(canvas: HtmlCanvasElement) -> Result<WebGlRenderer, Error> {
        get_canvas_context_1(&canvas)
            .ok_or(Error::from(NativeError::CanvasCreate))
            .map(|gl| WebGlRenderer {
                gl, 
                canvas,
                last_width: 0,
                last_height: 0
            })
    }

    pub fn context(self:&Self) -> &WebGlRenderingContext {
        &self.gl
    }

    pub fn resize(self:&mut Self, width:u32, height:u32) {
        if self.last_width != width || self.last_height != height {
            let canvas = &mut self.canvas;
            let gl = &mut self.gl;
            canvas.set_width(width);
            canvas.set_height(height);
            gl.viewport(0, 0, gl.drawing_buffer_width(), gl.drawing_buffer_height());
            self.last_width = width;
            self.last_height = height;
        }
    }
}

