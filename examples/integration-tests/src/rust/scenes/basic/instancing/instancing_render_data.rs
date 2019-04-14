use crate::rust::helpers::data::*;
use crate::rust::helpers::matrix::*;
use pure3d_webgl::errors::*;
use pure3d_webgl::enums::{BufferTarget, BufferUsage, DataType, PixelFormat};
use pure3d_webgl::renderer::WebGlRenderer;
use pure3d_webgl::texture::{assign_simple_texture, SimpleTextureOptions, WebGlTextureSource};
use pure3d_webgl::*;
use web_sys_loaders::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlRenderingContext, WebGlProgram, WebGlBuffer, HtmlImageElement, WebGlTexture};
use wasm_bindgen_futures::{future_to_promise, spawn_local, JsFuture};
use futures::future::{Future};
use super::instancing_data::*;

pub struct InstancingRenderData {
    pub scale_matrix:[f32;16],
    pub mvp_matrix:[f32;16],
    pub program:WebGlProgram,
    pub texture:WebGlTexture,
}

impl InstancingRenderData {
    pub fn new(webgl_renderer:&mut WebGlRenderer, instance_data:&InstancingInstanceData) -> Result<InstancingRenderData, Error> {
        let gl = webgl_renderer.context_mut();
        let program = create_program(&gl)?;
        let buffer = upload_data_to_buffer(&gl)?;
        let texture = gl.create_texture().unwrap();
        assign_buffer_to_attribute(&gl, &program, &buffer)?;
        assign_simple_texture(&gl, 
                              &SimpleTextureOptions{
                                  pixelFormat: PixelFormat::Rgba,
                                  ..SimpleTextureOptions::default()
                              }, 
                              &WebGlTextureSource::ImageElement(&instance_data.img), 
                              &texture
        )?;

        //instancing setup....
        

        //scale is constant for all bunnies
        let mut scale_matrix = [0.0;16];
        write_scale_matrix(instance_data.area.width, instance_data.area.height, 1.0, &mut scale_matrix);

        Ok(InstancingRenderData{
            program,
            scale_matrix, 
            mvp_matrix: [0.0;16], 
            texture
        })
    }

    pub fn update(self:&mut Self, camera_matrix:&[f32;16], area:&Area, pos:&Point) {
        let mut scratch_matrix:[f32;16] = [0.0;16]; 
        let InstancingRenderData {scale_matrix, mvp_matrix, ..} = self;

        //model-view-projection
        write_position_matrix(pos.x, pos.y, 0.0, &mut scratch_matrix);
        write_multiply_matrix(camera_matrix, &scratch_matrix, mvp_matrix); 

    }
}

/*
 * Everything below is just initial renderer setup
 */
fn create_program (gl:&WebGlRenderingContext) -> Result<WebGlProgram, Error> {
    shader::compile_shader(&gl, 
        include_str!("shaders/Instancing-Vertex.glsl"),
        include_str!("shaders/Instancing-Fragment.glsl")
    )
}

fn upload_data_to_buffer(gl:&WebGlRenderingContext) -> Result<WebGlBuffer, Error> {
    gl.create_buffer()
        .map_or(Err(Error::from("Couldn't create buffer")), |buffer| {
            let data:Vec<f32> = vec![  
                    0.0,1.0, // top-left
                    0.0,0.0, //bottom-left
                    1.0, 1.0, // top-right
                    1.0, 0.0 // bottom-right
            ];

            buffer::upload_array_buffer(&gl, &data, &BufferTarget::ArrayBuffer, &BufferUsage::StaticDraw, &buffer)
                .map(move |_| buffer)
        })
}

fn assign_buffer_to_attribute(gl:&WebGlRenderingContext, program:&WebGlProgram, buffer:&WebGlBuffer) -> Result<(), Error> {
    
    gl.use_program(Some(&program));

    buffer::bind_buffer(&gl, &BufferTarget::ArrayBuffer, &buffer); 

    attributes::get_attribute_location(&gl, &program, "a_vertex")
        .map(|loc| {
            let opts = attributes::AttributeOptions::new(2, DataType::Float);
            attributes::activate_attribute(&gl, &loc, &opts);
        })
}
