#[derive(Copy,Clone)]
pub struct Point {
    pub x:f64,
    pub y:f64,
}

#[derive(Copy,Clone)]
pub struct Color {
    pub r:f64,
    pub g:f64,
    pub b:f64,
    pub a:f64,
}

impl Color {
    pub fn as_slice(self:&Self) -> [f32;4] {
        [self.r as f32, self.g as f32, self.b as f32, self.a as f32] 
    }
}

#[derive(Copy,Clone)]
pub struct Area {
    pub width:f64,
    pub height:f64,
}
