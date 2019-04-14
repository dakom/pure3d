extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlUniformLocation};
use super::enums::{DataType};
use super::errors::*;
use super::renderer::*; 
use super::errors::*;

pub fn get_uniform_location(gl:&WebGlRenderingContext, program:&WebGlProgram, name:&str) -> Result<WebGlUniformLocation, Error> {
    gl.get_uniform_location(&program, &name)
        .ok_or(Error::from(NativeError::UniformLocation))
}

