extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use web_sys::{WebGlProgram, WebGlRenderingContext};
use super::enums::{DataType};

//TODO - caching and stuff
//Also... api wants mutable slices, maybe abstract over that?
//
//
