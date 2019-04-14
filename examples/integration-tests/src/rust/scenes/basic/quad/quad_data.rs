use crate::rust::helpers::data::*;
use crate::rust::helpers::matrix::*;
use pure3d_webgl::errors::*;
use pure3d_webgl::enums::{BufferTarget, BufferUsage, DataType};
use pure3d_webgl::renderer::WebGlRenderer;
use pure3d_webgl::*;
use web_sys::{WebGlRenderingContext, WebGlProgram, WebGlBuffer};

pub struct QuadInstanceData {
    pub pos: Point,
    pub area: Area,
    pub color: Color,
    pub direction: f64,
}

impl QuadInstanceData {
    pub fn new() -> QuadInstanceData { 

        let pos = Point{x: 500.0, y: 500.0};
        let area = Area{width: 300.0, height: 100.0};
        let color = Color::new(1.0, 1.0, 0.0, 1.0);

        QuadInstanceData{
                pos, 
                area, 
                color, 
                direction: 0.05, 
        }
    }

    pub fn update(self:&mut Self, time_stamp:f64) {
        let color = &mut self.color;
        let direction = &mut (self.direction);
        color.r += *direction;
        if *direction > 0.0 {
            if color.r > 1.0 {
                color.r = 1.0;
                *direction *= -1.0;
            }
        } else {
            if color.r < 0.0 {
                color.r = 0.0;
                *direction *= -1.0;
            }
        }

    }

}

pub struct QuadRenderData {
    pub scale_matrix:[f32;16],
    pub mvp_matrix:[f32;16],
    pub color_vec:[f32;4], 
    pub program:WebGlProgram,
    
}

impl QuadRenderData {
    pub fn new(webgl_renderer:&mut WebGlRenderer) -> Result<QuadRenderData, Error> {
        let gl = webgl_renderer.context_mut();
        let program = create_program(&gl)?;
        let buffer = upload_data_to_buffer(&gl)?;
        assign_buffer_to_attribute(&gl, &program, &buffer)?;
        Ok(QuadRenderData{
            program,
            scale_matrix: [0.0;16], 
            mvp_matrix: [0.0;16], 
            color_vec: [0.0;4], 
        })
    }
    pub fn update(self:&mut Self, camera_matrix:&[f32;16], instance_data:&QuadInstanceData) {
        let mut scratch_matrix:[f32;16] = [0.0;16]; 
        let QuadRenderData {scale_matrix, mvp_matrix, color_vec, ..} = self;
        let QuadInstanceData {pos, area, color, ..} = instance_data;

        //scale
        write_scale_matrix(area.width, area.height, 1.0, scale_matrix);
       
        //model-view-projection
        write_position_matrix(pos.x, pos.y, 0.0, &mut scratch_matrix);
        write_multiply_matrix(camera_matrix, &scratch_matrix, mvp_matrix); 

        //color
        color.write_to_v32_4(color_vec);


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
        .map(|loc| {
            let opts = attributes::AttributeOptions::new(2, DataType::Float);
            attributes::activate_attribute(&gl, &loc, &opts);
        })
}
