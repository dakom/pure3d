use crate::rust::helpers::data::*;
use crate::rust::helpers::matrix::*;
use super::quad_texture_data::*;
use crate::rust::scenes::scene::{Scene};
use pure3d_webgl::enums::{BeginMode};
use pure3d_webgl::renderer::*;
use pure3d_webgl::errors::*;
use std::rc::Rc;
use std::cell::RefCell;
use futures::future::{Future, result};
use web_sys::{console};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

pub struct QuadTextureScene {
    webgl_renderer: Rc<RefCell<WebGlRenderer>>, 
    camera_matrix:[f32;16],
    instance_data:QuadTextureInstanceData,
    render_data:QuadTextureRenderData,
}

impl QuadTextureScene {
    pub fn new(webgl_renderer:Rc<RefCell<WebGlRenderer>>) -> impl Future<Item = Box<QuadTextureScene>, Error = Error> {
        QuadTextureInstanceData::new()
            .and_then(|instance_data| {
                //this must all be in its own scope since we can't take ownership of
                //webgl_renderer while the borrow is still active
                let render_data_result = {
                    webgl_renderer.try_borrow_mut()
                        .map_err(|s| Error::from(s.to_string()))
                        .and_then(|mut webgl_renderer_ref| {
                            QuadTextureRenderData::new(&mut webgl_renderer_ref, &instance_data)
                        })
                };

                result(render_data_result)
                    .map(|render_data| {
                        Box::new(QuadTextureScene{
                            webgl_renderer,
                            camera_matrix: [0.0;16],
                            instance_data,
                            render_data,
                        })
                    })
            })
    }
}

impl Scene for QuadTextureScene {
    fn id(self:&Self) -> &str {
        "quad_texture"
    }
    fn tick(self:&mut Self, time_stamp:f64) -> Result<(), Error> {
        self.instance_data.update(time_stamp);
        self.render_data.update(&self.camera_matrix, &self.instance_data);
        
        let mut webgl_renderer_ref = self.webgl_renderer.try_borrow_mut().map_err(|e| e.to_string())?;
        self.render(&mut webgl_renderer_ref);

        Ok(())
    }

    fn resize(self:&mut Self, window_width: u32, window_height: u32) -> Result<(), Error> {
        //Note - we could also get it from self.webgl_renderer.borrow_mut etc.
        write_ortho(0.0, window_width as f64, 0.0, window_height as f64, 0.0, 1.0, &mut self.camera_matrix);
        Ok(())
    }

}


impl WebGlRender for QuadTextureScene {
    fn render(self: &Self, webgl_renderer:&mut WebGlRenderer) {
        let gl = webgl_renderer.context();
        let render_data = &self.render_data; 

        //only because the gl.uniform calls require passing a mutable matrix
        //see https://github.com/rustwasm/wasm-bindgen/issues/1131
        //and https://github.com/rustwasm/wasm-bindgen/issues/1005

        let mut temp_mut_matrix:[f32;16] = [0.0;16];
        let mut temp_mut_vec4:[f32;4] = [0.0;4];

        //scale
        temp_mut_matrix.copy_from_slice(&render_data.scale_matrix);
        let loc = gl.get_uniform_location(&render_data.program, "u_size");
        gl.uniform_matrix4fv_with_f32_array(loc.as_ref(), false, &mut temp_mut_matrix);

        //model-view-projection
        temp_mut_matrix.copy_from_slice(&render_data.mvp_matrix);
        let loc = gl.get_uniform_location(&render_data.program, "u_modelViewProjection");
        gl.uniform_matrix4fv_with_f32_array(loc.as_ref(), false, &mut temp_mut_matrix);

        //color
        temp_mut_vec4.copy_from_slice(&render_data.color_vec);
        let loc = gl.get_uniform_location(&render_data.program, "u_color");
        gl.uniform4fv_with_f32_array(loc.as_ref(), &mut temp_mut_vec4);
       
        //draw!
        gl.draw_arrays(BeginMode::TriangleStrip as u32, 0, 4);
    }
}
