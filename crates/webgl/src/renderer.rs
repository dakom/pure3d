extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::{console};
use web_sys::{HtmlCanvasElement, WebGlProgram, WebGlRenderingContext, WebGlUniformLocation};
use super::enums::{DataType};
use super::errors::*;
use super::canvas;
use super::extensions;
use super::shader;
use super::attributes;
use wasm_bindgen::JsCast;

pub struct WebGlRenderer <'a> {
    gl:WebGlRenderingContext,
    canvas: HtmlCanvasElement,
    last_width: u32,
    last_height: u32,

    last_program_id: u64,
    current_program_info: Option<&'a ProgramInfo<'a>>,
    program_info_lookup: HashMap<u64, ProgramInfo<'a>>,

    extension_lookup: HashMap<&'a str, js_sys::Object>,
}

struct ProgramInfo <'a> {
    pub id:u64,
    pub program: WebGlProgram,
    pub attribute_lookup: HashMap<&'a str, u32>,
    pub uniform_lookup: HashMap<&'a str, WebGlUniformLocation>
}


impl<'a> Drop for WebGlRenderer<'a> {
    fn drop(self:&mut Self) {
        self.gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT); 
        //console::log_1(&JsValue::from_str("Freed GL context!!!"));
    }
}

pub trait WebGlRender {
    fn render(&self, webgl_renderer:&mut WebGlRenderer) -> Result<(), Error>;
}

impl<'a> WebGlRenderer<'a> {
    //Canvas and Context
    pub fn new(canvas: HtmlCanvasElement) -> Result<WebGlRenderer<'a>, Error> {
        canvas::get_canvas_context_1(&canvas)
            .ok_or(Error::from(NativeError::CanvasCreate))
            .map(|gl| WebGlRenderer {
                gl, 
                canvas,
                last_width: 0,
                last_height: 0,
                last_program_id: 0,
                current_program_info: None,
                program_info_lookup: HashMap::new(),
                extension_lookup: HashMap::new()
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


    //SHADERS

    pub fn compile_program(self:&'a mut Self, vertex:&str, fragment:&str) -> Result<u64, Error> {
        let program = shader::compile_program(&self.gl, &vertex, &fragment)?;

        self.last_program_id += 1;
        let id = self.last_program_id; 

        let program_info = ProgramInfo {
            id,
            program,
            attribute_lookup: HashMap::new(),
            uniform_lookup: HashMap::new() 
        };

        self.program_info_lookup.insert(id, program_info);
        self.activate_program(id)?;

        Ok(id)
    }

    fn get_current_program_info(self:&'a Self) -> Result<&'a ProgramInfo, Error> {
        self.current_program_info
            .ok_or(Error::from(NativeError::MissingShaderProgram))
    }

    pub fn activate_program(self:&'a mut Self, program_id: u64) -> Result<(), Error> {

        let requires_setting = 
            match self.current_program_info {
                Some(info) => {
                    info.id != program_id
                },
                None => true
            };

        if requires_setting {
            let program_info = 
                self.program_info_lookup
                    .get(&program_id)
                    .ok_or(Error::from(NativeError::MissingShaderProgram))?;

            self.current_program_info = Some(program_info);

            let program = self.current_program_info.map(|info| &info.program);

            self.gl.use_program(program);
        }
        Ok(())
    }

    //ATTRIBUTES
    pub fn get_attribute_location(self:&'a mut Self, name:&str) -> Result<u32, Error> {


        let program_info = self.get_current_program_info()?;

        let (loc, requires_add) = match program_info.attribute_lookup.get(&name) {
            Some(loc) => {
                (*loc, false)
            },

            None => {
                let gl = self.context();
                let loc = attributes::get_attribute_location(&gl, &program_info.program, &name)?;
                (loc, true)
            }
        };

        if requires_add {
            self.current_program_info
                .ok_or(Error::from(NativeError::MissingShaderProgram))
                .as_mut()
                .map(|program_info| {
                    program_info.attribute_lookup.insert(&name, loc);
                });
        }

        Ok(loc)
    }

    pub fn activate_attribute_loc(self:&Self, loc:u32, opts:&attributes::AttributeOptions) {
        self.gl.vertex_attrib_pointer_with_f64(loc, opts.size, opts.data_type as u32, opts.normalized, opts.stride, opts.offset as f64);
        self.gl.enable_vertex_attrib_array(loc);
    }

    pub fn activate_attribute_name(self:&'a mut Self, name:&str, opts:&attributes::AttributeOptions) -> Result<(), Error> {
        let loc = self.get_attribute_location(&name)?;

        self.activate_attribute_loc(loc, &opts);

        Ok(())
    }

    //EXTENSIONS
    fn get_extension(self:&'a mut Self, name:&'a str) -> Result<&js_sys::Object, Error> {
        match self.extension_lookup.get(&name) {
            Some(ext) => Ok(ext),
            None => {
                let ext = extensions::get_extension(&self.gl, &name)?;
                self.extension_lookup.insert(&name, ext);
                self.extension_lookup.get(&name).ok_or(Error::from(NativeError::NoExtension))
            }
        }
    }

    pub fn get_extension_instanced_arrays(self:&'a mut Self) -> Result<extensions::AngleInstancedArrays, Error> {
        self.get_extension("ANGLE_instanced_arrays")
            .map(|ext| ext.unchecked_into::<extensions::AngleInstancedArrays>())
    }

    //TODO - uniforms and textures
}

