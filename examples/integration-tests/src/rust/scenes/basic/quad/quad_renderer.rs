use crate::rust::helpers::data::*;
use crate::rust::helpers::matrix::*;
use super::quad_data::*;
use pure3d_webgl::errors::*;
use pure3d_webgl::enums::{BeginMode, BufferTarget, BufferUsage, DataType};
use pure3d_webgl::renderer::WebGlRenderer;
use pure3d_webgl::*;
use web_sys::{WebGlRenderingContext, WebGlProgram, WebGlBuffer};

pub struct QuadRenderer {
    program: WebGlProgram,

    camera_matrix:[f32;16],
    scratch_matrix:[f32;16],
    scratch_matrix_2:[f32;16],

    //TODO - make part of global uniform uploading i.e. in WebGlRenderer
    vec4:[f32;4], 
}

impl QuadRenderer {
    pub fn new(renderer:&WebGlRenderer, window_size:&Area) -> Result<QuadRenderer, Error> {
        let gl = renderer.context();
        let program = create_program(&gl)?;
        let buffer = upload_data_to_buffer(&gl)?;
        assign_buffer_to_attribute(&gl, &program, &buffer)?;
        let mut camera_matrix:[f32;16] = [0.0;16];
        write_ortho(0.0, window_size.width, 0.0, window_size.height, 0.0, 1.0, &mut camera_matrix);
        Ok(QuadRenderer{
            program, 
            vec4: [0.0;4], 
            camera_matrix, 
            scratch_matrix: [0.0;16], 
            scratch_matrix_2: [0.0;16]
        })
    }

    pub fn render(self:&mut Self, renderer:&WebGlRenderer, data:&QuadData) {
        let gl = renderer.context();
     
        //scale
        write_scale_matrix(data.area.width, data.area.height, 1.0, &mut self.scratch_matrix);
        let loc = gl.get_uniform_location(&self.program, "u_size");
        gl.uniform_matrix4fv_with_f32_array(loc.as_ref(), false, &mut self.scratch_matrix);
       
        //position
        write_position_matrix(data.pos.x, data.pos.y, 0.0, &mut self.scratch_matrix);

        //model-view-projection
        write_multiply_matrix(&self.camera_matrix, &self.scratch_matrix, &mut self.scratch_matrix_2); 
        let loc = gl.get_uniform_location(&self.program, "u_modelViewProjection");
        gl.uniform_matrix4fv_with_f32_array(loc.as_ref(), false, &mut self.scratch_matrix_2);

        //color
        data.color.write_to_v32_4(&mut self.vec4);
        let loc = gl.get_uniform_location(&self.program, "u_color");
        gl.uniform4fv_with_f32_array(loc.as_ref(), &mut self.vec4);
       
        //draw!
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
