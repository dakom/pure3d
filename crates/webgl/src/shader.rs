extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use web_sys::{WebGlRenderingContext, WebGlProgram};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::JsValue;

pub fn compile_shader(gl:&WebGlRenderingContext, vertex:&str, fragment:&str) -> Result<WebGlProgram, &'static str> {
    gl.create_program()
        .ok_or("uhoh!")
        .and_then(|p:WebGlProgram| {
            compile_source (&gl, p, vertex, WebGlRenderingContext::VERTEX_SHADER)
        })
        .and_then(|p:WebGlProgram| {
            compile_source (&gl, p, fragment, WebGlRenderingContext::FRAGMENT_SHADER)
        })
        .and_then(|p:WebGlProgram| {
            gl.link_program(&p);
            let status = gl.get_program_parameter(&p, WebGlRenderingContext::LINK_STATUS);
            Ok(p)
        })
}

fn compile_source (gl:&WebGlRenderingContext, program:WebGlProgram, source: &str, source_type:u32) -> Result<WebGlProgram, &'static str> {
    let option_shader = gl.create_shader(source_type);

    match option_shader {
        Some(shader) => {
            gl.shader_source(&shader, source);
            gl.compile_shader(&shader);
            let status = gl.get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS);
            Ok(program)

            //TODO - make sure we can actually see errors
            //Fail on purpose with bad shader code
            /*
            if(status == JsValue::UNDEFINED) {
                Ok(program)
            } else {
                match gl.get_shader_info_log(&shader) {
                    Some(msg) => Err("There is an error message"),
                    None => Err("bad compile")
                }
            }
            */
        }
        None => Err("bad shader!")
    }
}
