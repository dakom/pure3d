extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use web_sys::{console};
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

impl Drop for WebGlRenderer {
    fn drop(self:&mut Self) {
        self.gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT); 
        //console::log_1(&JsValue::from_str("Freed GL context!!!"));
    }
}

pub trait WebGlRender {
    fn render(&self, webgl_renderer:&mut WebGlRenderer) -> Result<(), Error>;
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

    pub fn context_mut(self:&mut Self) -> &mut WebGlRenderingContext {
        &mut self.gl
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

    pub fn current_size(self:&Self) -> (u32, u32) {
        (self.last_width, self.last_height)
    }

    pub fn get_extension(self:&mut Self, name:&str) -> Result<js_sys::Object, Error> {
        match self.gl.get_extension(name) {
            Ok(obj) => obj.ok_or(Error::from(NativeError::NoExtension)),
            Err(err) => Err(Error::from(err))
        }
    }
}

