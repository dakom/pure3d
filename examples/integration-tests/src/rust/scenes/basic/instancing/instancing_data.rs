use crate::rust::helpers::data::*;
use crate::rust::helpers::matrix::*;
use pure3d_webgl::errors::*;
use pure3d_webgl::enums::{BufferTarget, BufferUsage, DataType, PixelFormat};
use pure3d_webgl::renderer::WebGlRenderer;
use pure3d_webgl::texture::{assign_simple_texture, SimpleTextureOptions, WebGlTextureSource};
use pure3d_webgl::*;
use web_sys_loaders::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlRenderingContext, WebGlProgram, WebGlBuffer, HtmlImageElement, WebGlTexture};
use wasm_bindgen_futures::{future_to_promise, spawn_local, JsFuture};
use futures::future::{Future};
use rand::rngs::{OsRng};
use rand::{Rng};

pub struct InstancingInstanceData {
    pub bunnies: Vec<Bunny>,
    pub area: Area,
    pub img: HtmlImageElement,
}

pub struct Bunny {
    pub pos: Point,
    pub traj: Point,
    pub speed: f64
}

impl Bunny {
    pub fn new(pos: Point) -> Self {
        let mut rng = OsRng::new().unwrap();

        let mut traj = Point {
                x: rng.gen(),
                y: rng.gen() 
        };

        let mut flip:bool = rng.gen();

        if flip {
            traj.x *= -0.1;
        }
        
        flip = rng.gen();

        if flip {
            traj.y *= -0.1;
        }

        Self {
            pos,
            traj,
            speed: 100.0
        }
    }
}

impl InstancingInstanceData {
    pub fn new() -> impl Future<Item = InstancingInstanceData, Error = Error> { 
        image::fetch_image(String::from("http://localhost:31337/sprites/bunnies/bunny.png"))
            .map_err(Error::from)
            .map(|img| {

                let bunnies = vec![
                    Bunny::new(
                        Point{ x: 200.0, y: 500.0},
                    ),
                    Bunny::new(
                        Point{ x: 500.0, y: 500.0},
                    )
                ];

                let area = Area{width: 25.0, height: 32.0};

                InstancingInstanceData{
                        bunnies, 
                        area, 
                        img,
                }
            })
    }

    pub fn update(self:&mut Self, time_stamp:f64) {
        for bunny in &mut self.bunnies {
            bunny.pos.x += (bunny.traj.x * time_stamp * bunny.speed);
            bunny.pos.y += (bunny.traj.y * time_stamp * bunny.speed);
        }
    }
}

