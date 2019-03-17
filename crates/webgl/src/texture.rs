extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use web_sys::{WebGlRenderingContext, WebGlTexture, ImageBitmap, ImageData, HtmlImageElement, HtmlCanvasElement, HtmlVideoElement};
use wasm_bindgen::prelude::JsValue;
use wasm_bindgen::JsCast;
use js_sys::{Object};
use super::errors::*;
use super::enums::{TextureWrapMode, TextureMinFilter, TextureMagFilter};

pub enum WebGlTextureSource <'a> {
    ArrayBufferView(Object, i32, i32),
    ByteArray(&'a mut [u8], i32, i32),
    ImageBitmap(&'a ImageBitmap),
    ImageData(&'a ImageData),
    ImageElement(&'a HtmlImageElement),
    CanvasElement(&'a HtmlCanvasElement),
    VideoElement(&'a HtmlVideoElement),
}

pub struct SimpleTextureOptions {
    useMips: bool,
    flipY: bool,
    wrapS: TextureWrapMode,
    wrapT: TextureWrapMode,
    filterMin: TextureMinFilter,
    filterMag: TextureMagFilter
}

impl Default for SimpleTextureOptions {
    fn default() -> Self {
        Self {
            useMips: true,
            flipY: true,
            wrapS: TextureWrapMode::ClampToEdge,
            wrapT: TextureWrapMode::ClampToEdge,
            filterMin: TextureMinFilter::Linear,
            filterMag: TextureMagFilter::Linear,
        }
    }
}

pub struct TextureOptions {
}

pub fn assign_simple_texture <'a> (gl:&WebGlRenderingContext, opts:&SimpleTextureOptions, src:&WebGlTextureSource, dest:&'a WebGlTexture) -> Result<&'a WebGlTexture, Error> {
    assign_texture(&gl, &get_texture_options(&opts), &src, &dest)
}


pub fn assign_texture <'a>(gl:&WebGlRenderingContext, opts:&TextureOptions, src:&WebGlTextureSource, dest:&'a WebGlTexture) -> Result<&'a WebGlTexture, Error> {
    //TOIDO - build this!
    Ok(dest)
}

fn get_texture_options(opts:&SimpleTextureOptions) -> TextureOptions {
    TextureOptions{}
}
