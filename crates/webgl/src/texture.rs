extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use web_sys::{WebGlRenderingContext, WebGlTexture};
use wasm_bindgen::prelude::JsValue;
use super::errors::*;

enum WebGlTextureSource {
    ArrayBufferView(Object, i32, i32),
    ByteArray(&mut [u8], i32, i32),
    ImageBitmap(&ImageBitmap),
    ImageData(&ImageData),
    ImageElement(&HtmlImageElement),
    CanvasElement(&HtmlCanvasElement),
    VideoElement(&HtmlVideoElement),
}

struct SimpleTextureOptions {
    useMips: bool,
    flipY: bool,
    wrapS: TextureWrapMode,
    wrapT: TextureWrapMode,
    filterMin: TextureMinFilter,
    filterMag: TextureMagFilter
}

impl Default for SimpleTextureOptions {
    fn default() -> Self {
        useMips: true,
        flipY: true,
        wrapS: TextureWrapMode::ClampToEdge,
        wrapT: TextureWrapMode::ClampToEdge,
        filterMin: TextureMinFilter::Linear,
        filterMag: TextureMagFilter::Linear,
    }
}

pub fn create_simple_texture (gl:&WebGlRenderingContext, opts:&SimpleTextureOptions, src:&WebGlTextureSource) -> Result<WebGlTexture, Error> {
    create_texture(&gl, &get_texture_options(&opts), &src)
}


pub fn create_texture (gl:&WebGlRenderingContext, opts:&TextureOptions, src:&WebGlTextureSource) -> Result<WebGlTexture, Error> {

}
