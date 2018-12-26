#[derive(Copy,Clone)]
pub struct Point {
    pub x:f64,
    pub y:f64,
}

#[derive(Copy,Clone)]
#[repr(C)]
pub struct Color {
    pub r:f64,
    pub g:f64,
    pub b:f64,
    pub a:f64,

    _values:[f32;4]
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Color {
        Color {
            r,
            g,
            b,
            a,
            _values: [0.0;4]
        }
    }

    pub fn values(self:&mut Self) -> &mut[f32] {
        self._values[0] = self.r as f32;
        self._values[1] = self.g as f32;
        self._values[2] = self.b as f32;
        self._values[3] = self.a as f32;

        &mut self._values
    }
}

#[derive(Copy,Clone)]
pub struct Area {
    pub width:f64,
    pub height:f64,
}


