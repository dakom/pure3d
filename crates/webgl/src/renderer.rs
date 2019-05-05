extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use std::collections::HashMap;
use std::collections::hash_map::OccupiedEntry;
use std::collections::hash_map::VacantEntry;
use std::collections::hash_map::Entry;
use wasm_bindgen::prelude::*;
use web_sys::{console};
use web_sys::{HtmlCanvasElement, WebGlProgram, WebGlBuffer, WebGlRenderingContext, WebGlUniformLocation};
use super::enums::{DataType, BufferTarget, BufferUsage};
use super::errors::*;
use super::canvas;
use super::extensions;
use super::shader;
use super::attributes;
use super::buffers;
use super::uniforms;
use wasm_bindgen::JsCast;

pub struct WebGlRenderer <'a> {
    gl:WebGlRenderingContext,
    canvas: HtmlCanvasElement,
    last_width: u32,
    last_height: u32,

    current_program_id: Option<u64>,
    program_info_lookup: HashMap<u64, ProgramInfo<'a>>,
    current_buffer_id: Option<u64>,
    current_buffer_target: Option<BufferTarget>,
    buffer_lookup: HashMap<u64, WebGlBuffer>,
    extension_lookup: HashMap<&'a str, js_sys::Object>,
    global_attribute_lookup: HashMap<&'a str, u32>,
}

struct ProgramInfo <'a> {
    pub id:u64,
    pub program: WebGlProgram,
    pub attribute_lookup: HashMap<&'a str, u32>,
    pub uniform_lookup: HashMap<&'a str, WebGlUniformLocation>
}


impl<'a> Drop for WebGlRenderer<'a> {
    fn drop(&mut self) {
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
                current_program_id: None,
                current_buffer_id: None,
                current_buffer_target: None,
                program_info_lookup: HashMap::new(),
                buffer_lookup: HashMap::new(),
                extension_lookup: HashMap::new(),
                global_attribute_lookup: HashMap::new()
            })
    }

    pub fn context(&self) -> &WebGlRenderingContext {
        &self.gl
    }

    pub fn context_mut(&mut self) -> &mut WebGlRenderingContext {
        &mut self.gl
    }

    pub fn resize(&mut self, width:u32, height:u32) {
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

    pub fn current_size(&self) -> (u32, u32) {
        (self.last_width, self.last_height)
    }

    //SHADERS

    pub fn compile_program(&mut self, vertex:&str, fragment:&str) -> Result<u64, Error> {
        let program = shader::compile_program(&self.gl, &vertex, &fragment)?;

        let id = self.program_info_lookup.keys().max().map_or(0, |x| x + 1);

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

    fn get_current_program_info(&self) -> Result<&'a ProgramInfo, Error> {
        self.current_program_id
            .and_then(|id| self.program_info_lookup.get(&id))
            .ok_or(Error::from(NativeError::MissingShaderProgram))
    }


    pub fn activate_program(&mut self, program_id: u64) -> Result<(), Error> {
        if Some(program_id) != self.current_program_id {
            self.current_program_id = Some(program_id);
            self.get_current_program_info()
                .map(|program_info| {
                    self.gl.use_program(Some(&program_info.program));
                })
        } else {
            Ok(())
        }
    }

    //BUFFERS
    pub fn create_buffer(&mut self) -> Result<u64, Error> {

        let buffer = self.gl.create_buffer()
            .ok_or(Error::from(NativeError::NoCreateBuffer))?;

        let id = self.buffer_lookup.keys().max().map_or(0, |x| x + 1);
        
        self.buffer_lookup.insert(id, buffer);

        Ok(id)
    }

    pub fn activate_buffer(&mut self, id:u64, target: BufferTarget) -> Result<(), Error> {
        let buffer = self.buffer_lookup.get(&id)
            .ok_or(Error::from(NativeError::NoExistingBuffer))?;

        if Some(id) != self.current_buffer_id || Some(target) != self.current_buffer_target {
            buffers::bind_buffer(&self.gl, target, &buffer);
            self.current_buffer_id = Some(id);
            self.current_buffer_target = Some(target);
        }

        Ok(())
    }

    pub fn upload_array_buffer(&mut self, id:u64, values:&[f32], target: BufferTarget, usage:BufferUsage) -> Result<(), Error> {
        self.activate_buffer(id, target)?;

        buffers::upload_array_buffer(&self.gl, &values, target, usage, self.buffer_lookup.get(&id).unwrap())
    }

    //ATTRIBUTES
    pub fn get_attribute_location_from_current_program(&mut self, name:&'a str) -> Result<u32, Error> 
    
    {
        let program_id = self.current_program_id.ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = self.program_info_lookup.get_mut(&program_id).unwrap(); //we already know this is okay

        let entry = program_info.attribute_lookup.entry(&name);

        match entry {
            Entry::Occupied(entry) => Ok(*entry.into_mut()),
            Entry::Vacant(entry) => {
                let loc = attributes::get_attribute_location(&self.gl, &program_info.program, &name)?;
                Ok(*entry.insert(loc))
            }
        }
    }

    //TODO: pub fn get_attribute_location_from_global(&mut self, name:&'a str) -> Result<u32, Error> 

    pub fn activate_attribute_loc(&mut self, loc:u32, opts:&attributes::AttributeOptions) {
        self.gl.vertex_attrib_pointer_with_f64(loc, opts.size, opts.data_type as u32, opts.normalized, opts.stride, opts.offset as f64);
        self.gl.enable_vertex_attrib_array(loc);
    }

    pub fn activate_attribute_name_in_current_program(&mut self, name:&'a str, opts:&attributes::AttributeOptions) -> Result<(), Error> {
        let loc = self.get_attribute_location_from_current_program(&name)?;

        self.activate_attribute_loc(loc, &opts);

        Ok(())
    }

    //EXTENSIONS
    fn get_extension(&mut self, name:&'a str) -> Result<&js_sys::Object, Error> {
        let entry = self.extension_lookup.entry(&name);

        match entry {
            Entry::Occupied(entry) => Ok(entry.into_mut()),
            Entry::Vacant(entry) => {
               let ext = extensions::get_extension(&self.gl, &name)?;
               Ok(entry.insert(ext))
            }
        }
    }

    pub fn get_extension_instanced_arrays(&mut self) -> Result<&extensions::AngleInstancedArrays, Error> {
        self.get_extension("ANGLE_instanced_arrays")
            .map(|ext| ext.unchecked_ref::<extensions::AngleInstancedArrays>())
    }

    //UNIFORMS
    pub fn get_uniform_location_in_current_program(&mut self, name:&'a str) -> Result<&WebGlUniformLocation, Error> {

        let program_id = self.current_program_id.ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = self.program_info_lookup.get_mut(&program_id).unwrap(); //we already know this is okay

        let entry = program_info.uniform_lookup.entry(&name);

        match entry {
            Entry::Occupied(entry) => Ok(entry.into_mut()),
            Entry::Vacant(entry) => {
               let loc = uniforms::get_uniform_location(&self.gl, &program_info.program, &name)?;
               Ok(entry.insert(loc))
            }
        }
    }

    pub fn set_uniform_data_in_current_program(&mut self, name:&'a str, transpose: bool, data: &uniforms::UniformData) -> Result<(), Error> {
        let loc = self.get_uniform_location_in_current_program(&name)?.clone(); //meh... it's just a number, I think...
        self.set_uniform_data_loc(&loc, transpose, &data);
        Ok(())
    }

    pub fn set_uniform_data_loc(&self, loc:&WebGlUniformLocation, transpose: bool, data: &uniforms::UniformData) {
        //Maybe compare to local cache?
        uniforms::set_uniform_data(&self.gl, &loc, transpose, &data);
    }

    //TEXTURES - todo
}

