pub fn write_vf64_vf32_4(values:&[f64], target:&mut [f32]) {
    target[0] = values[0] as f32;
    target[1] = values[1] as f32;
    target[2] = values[2] as f32;
    target[3] = values[3] as f32;
}

pub struct Point {
    pub x:f64,
    pub y:f64,
}

#[repr(C)]
pub struct Color {
    pub r:f64,
    pub g:f64,
    pub b:f64,
    pub a:f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Color {
        Color {
            r,
            g,
            b,
            a,
        }
    }

    pub fn values(self:&Self) -> &[f64] {
        let pointer = self as *const Color as *const f64;
        let slice: &[f64] = unsafe { std::slice::from_raw_parts(pointer, 4) };
        slice
    }
}

pub struct Area {
    pub width:f64,
    pub height:f64,
}


