extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use web_sys::{WebGlRenderingContext, WebGlProgram, WebGlShader};
use wasm_bindgen::prelude::JsValue;

/*
 * TODO
 * 1. Have cleanup_shaders() to detatch and delete shader
 * 2. Have cleanup_program() to call cleanup_shaders() and also free program
 * 3. See if we can impl Drop for WebGlProgram
 * 4. Setup global attribute locations?
 */


struct CompileSteps {
    program: Option<WebGlProgram>,
    fragment: Option<WebGlShader>,
    vertex: Option<WebGlShader>,
    error_message: String,
}

impl CompileSteps {
    pub fn new() -> CompileSteps {
        CompileSteps {
            program: None,
            fragment: None,
            vertex: None,
            error_message: String::from("unknown error!")
        }
    }

    pub fn free_shaders(&mut self, gl:&WebGlRenderingContext) {
        let free_shader = |s:Option<&WebGlShader>| {
            s.map(|shader:&WebGlShader| {
                //if the shader exists, the program had to have been valid
                gl.detach_shader(self.program.as_ref().unwrap(), shader);
            });
            gl.delete_shader(s);
        };

        free_shader(self.fragment.as_ref()); 
        free_shader(self.vertex.as_ref()); 

        self.fragment = None;
        self.vertex = None;
    }

    pub fn free_all(&mut self, gl:&WebGlRenderingContext) {
        self.free_shaders(gl);

        gl.delete_program(self.program.as_ref());
        self.program = None;
    }
}


pub fn compile_shader(gl:&WebGlRenderingContext, vertex:&str, fragment:&str) -> Result<WebGlProgram, String> {
    let result = create_program(&gl, CompileSteps::new())
        .and_then(|compile_steps:CompileSteps|
            compile_source(&gl, compile_steps, fragment, WebGlRenderingContext::FRAGMENT_SHADER)
        )
        .and_then(|compile_steps:CompileSteps|
            compile_source(&gl, compile_steps, vertex, WebGlRenderingContext::VERTEX_SHADER)
        )
        .and_then(|compile_steps:CompileSteps|
            link_program(&gl, compile_steps)
        );

    match result {
        Ok(mut compile_steps) => {
            compile_steps.free_shaders(gl);
            Ok(compile_steps.program.unwrap())
        }
        Err(mut compile_steps) => {
            compile_steps.free_all(gl);
            Err(compile_steps.error_message)
        }
    }

}

fn create_program (gl:&WebGlRenderingContext, mut compile_steps:CompileSteps) -> Result<CompileSteps, CompileSteps> {
    match gl.create_program() {
        Some(program) => {
            compile_steps.program = Some(program);
            Ok(compile_steps)
        }
        None => {
            compile_steps.error_message = String::from("Couldn't create program (unknown error");
            Err(compile_steps)
        }
    }
}

fn compile_source (gl:&WebGlRenderingContext, mut compile_steps:CompileSteps, source: &str, source_type:u32) -> Result<CompileSteps, CompileSteps> {
    let option_shader = gl.create_shader(source_type);

    match option_shader {
        Some(shader) => {
            gl.shader_source(&shader, source);
            gl.compile_shader(&shader);
            match do_with_check( || gl.get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS), || gl.get_shader_info_log(&shader)) {
                Some(error_message) => {
                    compile_steps.error_message = error_message;
                    Err(compile_steps)
                }
                None => {
                    gl.attach_shader(&compile_steps.program.as_ref().unwrap(), &shader);
                    if source_type == WebGlRenderingContext::VERTEX_SHADER {
                        compile_steps.vertex = Some(shader);
                    } else {
                        compile_steps.fragment = Some(shader);
                    }
                    Ok(compile_steps)
                }
            }

        }
        None => {
            compile_steps.error_message = String::from("bad shader (unknown error");
            Err(compile_steps)
        }
    }
}

fn link_program (gl:&WebGlRenderingContext, mut compile_steps:CompileSteps) -> Result<CompileSteps, CompileSteps> { 
    let program = &compile_steps.program.as_ref().unwrap();
    gl.link_program(program);

    match do_with_check( || gl.get_program_parameter(program, WebGlRenderingContext::LINK_STATUS), || gl.get_program_info_log(program)) {
        Some(error_message) => {
            compile_steps.error_message = error_message;
            Err(compile_steps)
        }
        None => Ok(compile_steps)
    }
}


fn do_with_check <T,U>(set_status: T, get_status: U) -> Option<String> 
    where T: Fn() -> JsValue, U: Fn() -> Option<String>, 
{

    if set_status() == JsValue::FALSE {
        match get_status() {
            None => Some(String::from("unknown shader compiler error!")),
            err => err
        }
    } else {
        None
    }
}
