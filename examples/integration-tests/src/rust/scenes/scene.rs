use pure3d_webgl::renderer::WebGlRenderer;
use pure3d_webgl::errors::*;
use std::rc::Rc;
use std::cell::RefCell;

pub trait Scene {
    fn tick(&mut self, time_stamp:f64) -> Result<(), Error>;
    fn resize(&mut self, width: u32, height: u32) -> Result<(), Error>;
    fn should_stop(&mut self) -> bool;
}
