use pure3d_webgl::renderer::WebGlRenderer;
use pure3d_webgl::errors::Error;
pub trait Scene {
    fn render(&mut self, renderer:&WebGlRenderer);
    fn update_data(&mut self, time_stamp:f64);
    fn update_renderer(&mut self, renderer:&mut WebGlRenderer);
    fn should_stop(&mut self) -> bool;
}
