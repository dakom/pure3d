extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use web_sys::{WebGlRenderingContext, WebGlProgram};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::JsValue;
use web_sys::console;

/*
 * TODO
 * 1. Have cleanup_shaders() to detatch and delete shader
 * 2. Have cleanup_program() to call cleanup_shaders() and also free program
 * 3. See if we can impl Drop for WebGlProgram
 * 4. Setup global attribute locations?
 */

pub fn compile_shader(gl:&WebGlRenderingContext, vertex:&str, fragment:&str) -> Result<WebGlProgram, String> {
    gl.create_program()
        .ok_or(String::from("Couldn't create WebGlProgram"))
        .and_then(|p:WebGlProgram| {
            compile_source (&gl, p, vertex, WebGlRenderingContext::VERTEX_SHADER)
        })
        .and_then(|p:WebGlProgram| {
            compile_source (&gl, p, fragment, WebGlRenderingContext::FRAGMENT_SHADER)
        })
        .and_then(|p:WebGlProgram| {
            link_program(&gl, p)
        })
}

fn compile_source (gl:&WebGlRenderingContext, program:WebGlProgram, source: &str, source_type:u32) -> Result<WebGlProgram, String> {
    let option_shader = gl.create_shader(source_type);

    match option_shader {
        Some(shader) => {
            gl.shader_source(&shader, source);
            gl.compile_shader(&shader);
            match do_with_check( || gl.get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS), || gl.get_shader_info_log(&shader)) {
                Some(error_message) => Err(error_message),
                None => {
                    gl.attach_shader(&program, &shader);
                    Ok(program)
                }
            }

        }
        None => Err(String::from("bad shader!"))
    }
}

fn link_program (gl:&WebGlRenderingContext, program:WebGlProgram) -> Result<WebGlProgram, String> {
    gl.link_program(&program);

    match do_with_check( || gl.get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS), || gl.get_program_info_log(&program)) {
        Some(error_message) => Err(error_message),
        None => Ok(program)
    }
}


fn do_with_check <T,U>(set_status: T, get_status: U) -> Option<String> 
    where T: Fn() -> JsValue, U: Fn() -> Option<String>, 
{

    if(set_status() == JsValue::FALSE) {
        match get_status() {
            None => Some(String::from("unknown shader compiler error!")),
            err => err
        }
    } else {
        None
    }
}
