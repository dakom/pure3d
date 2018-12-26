use super::helpers::*;
use pure3d_webgl::errors::*;
use pure3d_webgl::enums::{BeginMode, BufferTarget, BufferUsage, DataType};
use pure3d_webgl::renderer::WebGlRenderer;
use pure3d_webgl::*;
use web_sys::{WebGlRenderingContext, WebGlProgram, WebGlBuffer};

/*
 * QuadRenderer is tied to the rAF cycle
 * Quad should conceptually be able to be sent across threads
 * The ratio is 1:many, i.e. one QuadRenderer which gets passed many quads
 */
pub struct Quad {
    pub pos: Point,
    pub area: Area,
    pub color: Color,
}

impl Quad {
    pub fn new() -> Quad { 
        let pos = Point{x: 200.0, y: 200.0};
        let area = Area{width: 10.0, height: 100.0};
        let color = Color::new(1.0, 1.0, 0.0, 1.0);
        Quad{pos, area, color}
    }

    pub fn update(self:&mut Self, direction:f64) -> f64 {
        self.color.r += direction;
        if direction > 0.0 {
            if self.color.r > 1.0 {
                self.color.r = 1.0;
                return direction * -1.0;
            }
        } else {
            if self.color.r < 0.0 {
                self.color.r = 0.0;
                return direction * -1.0;
            }
        }
        direction
    }
}

pub struct QuadRenderer {
    renderer:WebGlRenderer,
    program: WebGlProgram
}

impl QuadRenderer {
    pub fn new(renderer:WebGlRenderer) -> Result<QuadRenderer, Error> {
        let gl = renderer.context();
        let program = create_program(&gl)?;
        let buffer = upload_data_to_buffer(&gl)?;
        assign_buffer_to_attribute(&gl, &program, &buffer)?;
        Ok(QuadRenderer{renderer, program})
    }
    pub fn render(self:&Self, quad:&mut Quad) {
        let gl = self.renderer.context();
        let loc = gl.get_uniform_location(&self.program, "u_color");

        gl.uniform4fv_with_f32_array(loc.as_ref(), quad.color.values());
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
