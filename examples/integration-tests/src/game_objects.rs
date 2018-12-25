use super::helpers::*;
use pure3d_webgl::enums::{BeginMode};

pub struct Quad {
    pub pos: Point,
    pub area: Area,
    pub color: Color,
}

impl Quad {
    pub fn new() -> Quad {
        let pos = Point{x: 200.0, y: 200.0};
        let area = Area{width: 10.0, height: 100.0};
        let color = Color{r:1.0, g: 1.0, b: 0.0, a: 1.0};
        Quad{pos, area, color}
    }

    pub fn update(self:&mut Self, direction:f32) -> f32 {
        self.color.r += direction;
        if(direction > 0.0) {
            if(self.color.r > 1.0) {
                self.color.r = 1.0;
                return direction * -1.0;
            }
        } else {
            if(self.color.r < 0.0) {
                self.color.r = 0.0;
                return direction * -1.0;
            }
        }
        direction
    }
    pub fn render(self:&Self, renderer:&Renderer) {
        let Renderer {gl, program} = renderer;

//fn change_quad(gl:&WebGlRenderingContext, program:&WebGlProgram, pos:&Point, area:&Area, color:&Color) {
        let loc = gl.get_uniform_location(&program, "u_color");

        //See if we can use struct as array
        //also, followup at https://github.com/rustwasm/wasm-bindgen/issues/1131
        let mut values = self.color.as_slice();
        gl.uniform4fv_with_f32_array(loc.as_ref(), &mut values);
        gl.draw_arrays(BeginMode::TriangleStrip as u32, 0, 4);
    }
}


