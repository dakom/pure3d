use pure3d_webgl::renderer::WebGlRenderer;
use pure3d_webgl::errors::Error;
pub trait Scene {
    fn render(&mut self);
    fn update(&mut self, time_stamp:f64);
    fn should_stop(&mut self) -> bool;
}
