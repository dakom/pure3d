extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use web_sys::{WebGlBuffer, WebGlRenderingContext, WebGlProgram};
use pure3d_webgl::*; 
use pure3d_webgl::errors::*;
use pure3d_webgl::enums::*;
use super::helpers::*;

pub fn setup_renderer(canvas_element:web_sys::HtmlCanvasElement) -> Result<Renderer, Error> {
    let gl = create_context(canvas_element)?;
    let program = create_program(&gl)?;
    let renderer = Renderer{gl, program};
    let buffer = upload_data_to_buffer(&renderer)?;
    assign_buffer_to_attribute(&buffer, &renderer)?;
    Ok(renderer)

        /*
        create_context(canvas_element)
            .and_then(|gl| create_program(&gl).map(|program| Renderer{gl, program}))
            .and_then(|renderer| upload_data_to_buffer(&renderer).map(|buffer| (buffer, renderer)))
            .and_then(|(buffer, renderer)| assign_buffer_to_attribute(&buffer, &renderer).map(|_| renderer))
        */
}

fn create_context (canvas_element: web_sys::HtmlCanvasElement) -> Result<WebGlRenderingContext, Error> {
    canvas::get_canvas_context(canvas_element, canvas::ContextType::Gl(canvas::WebGlVersion::One))
        .ok_or(Error::from(NativeError::CanvasCreate))
}

fn create_program (gl:&WebGlRenderingContext) -> Result<WebGlProgram, Error> {
    shader::compile_shader(&gl, 
        include_str!("shaders/Quad-Vertex.glsl"),
        include_str!("shaders/Quad-Fragment.glsl")
    )
}
fn upload_data_to_buffer(renderer:&Renderer) -> Result<WebGlBuffer, Error> {
    let Renderer {gl, program} = renderer;
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

fn assign_buffer_to_attribute(buffer:&WebGlBuffer, renderer:&Renderer) -> Result<(), Error> {
    let Renderer {gl, program} = renderer;
    
    gl.use_program(Some(&program));

    buffer::bind_buffer(&gl, &BufferTarget::ArrayBuffer, &buffer); 

    attributes::get_attribute_location(&gl, &program, "a_vertex")
        .ok_or(Error::from(NativeError::AttributeLocation))
        .map(|loc| {
            let opts = attributes::AttributeOptions::new(2, DataType::Float);
            attributes::activate_attribute(&gl, &loc, &opts);
        })
}
