use crate::rust::helpers::data::*;
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
    pub fn new(renderer:&WebGlRenderer, window_size:&Area) -> Result<QuadScene, Error> {
        let _renderer = QuadRenderer::new(&renderer, &window_size)?;
        let _data = QuadData::new();

        Ok(QuadScene{_renderer, _data})
    }
}

impl Scene for QuadScene {
    fn render(self:&mut Self, renderer:&WebGlRenderer) {
        self._renderer.render(&renderer, &self._data);
    }

    fn update_data(self:&mut Self, time_stamp:f64) {
        self._data.update(time_stamp);
    }

    fn update_renderer(self:&mut Self, renderer:&mut WebGlRenderer) {
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


