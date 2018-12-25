use web_sys::{WebGlRenderingContext, WebGlProgram};

pub struct Renderer {
    pub gl:WebGlRenderingContext,
    pub program:WebGlProgram
}

#[derive(Copy,Clone)]
pub struct Point {
    pub x:f64,
    pub y:f64,
}

#[derive(Copy,Clone)]
#[repr(C)]
pub struct Color {
    pub r:f32,
    pub g:f32,
    pub b:f32,
    pub a:f32,
}

impl Color {
    pub fn as_slice(self:&Self) -> Vec<f32> {
        vec![self.r, self.g, self.b, self.a]
        //TODO: make this work! and return &mut [f32]
        /*
        let pointer = self as *const Color as *const f32;
        let slice: &mut [f32] = unsafe { std::slice::from_raw_parts(pointer, 4) };
        slice
        */
    }
}

#[derive(Copy,Clone)]
pub struct Area {
    pub width:f64,
    pub height:f64,
}


