use crate::rust::helpers::data::*;
use super::quad_data::*;
use pure3d_webgl::errors::*;
use pure3d_webgl::enums::{BeginMode, BufferTarget, BufferUsage, DataType};
use pure3d_webgl::renderer::WebGlRenderer;
use pure3d_webgl::*;
use web_sys::{WebGlRenderingContext, WebGlProgram, WebGlBuffer};
pub struct QuadRenderer {
    renderer:WebGlRenderer,
    program: WebGlProgram,
    vec4:[f32;4]
}

impl QuadRenderer {
    pub fn new(renderer:WebGlRenderer) -> Result<QuadRenderer, Error> {
        let gl = renderer.context();
        let program = create_program(&gl)?;
        let buffer = upload_data_to_buffer(&gl)?;
        assign_buffer_to_attribute(&gl, &program, &buffer)?;
        Ok(QuadRenderer{renderer, program, vec4: [0.0;4]})
    }

    pub fn render(self:&mut Self, data:&QuadData) {
        let gl = self.renderer.context();
        let loc = gl.get_uniform_location(&self.program, "u_color");
        
        write_vf64_vf32_4(data.color.values(), &mut self.vec4);
        gl.uniform4fv_with_f32_array(loc.as_ref(), &mut self.vec4);
        gl.draw_arrays(BeginMode::TriangleStrip as u32, 0, 4);
    }
}

/*
 * Everything below is just initial renderer setup
 */
fn create_program (gl:&WebGlRenderingContext) -> Result<WebGlProgram, Error> {
    shader::compile_shader(&gl, 
        include_str!("shaders/Quad-Vertex.glsl"),
        include_str!("shaders/Quad-Fragment.glsl")
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
        .ok_or(Error::from(NativeError::AttributeLocation))
        .map(|loc| {
            let opts = attributes::AttributeOptions::new(2, DataType::Float);
            attributes::activate_attribute(&gl, &loc, &opts);
        })
}
