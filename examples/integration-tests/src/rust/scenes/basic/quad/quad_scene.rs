use super::quad_renderer::*;
use super::quad_data::*;
use crate::rust::scenes::scene::{Scene};
use pure3d_webgl::renderer::WebGlRenderer;
use pure3d_webgl::errors::*;

pub struct QuadScene {
    _renderer:QuadRenderer,
    _data:QuadData,
}

impl QuadScene {
    pub fn new(renderer:&WebGlRenderer) -> Result<QuadScene, Error> {
        let _renderer = QuadRenderer::new(renderer.clone())?;
        let _data = QuadData::new();

        Ok(QuadScene{_renderer, _data})
    }
}

impl Scene for QuadScene {
    fn render(self:&mut Self) {
        self._renderer.render(&self._data);
    }

    fn update(self:&mut Self, time_stamp:f64) {
        self._data.update(time_stamp);
    }

    fn should_stop(self:&mut Self) -> bool {
        false
        /*
        if self._data.current_time > 3000.0 {
            true
        } else {
            false
        }
        */
    }
}


