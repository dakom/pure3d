use crate::rust::helpers::data::*;

pub struct QuadData {
    pub pos: Point,
    pub area: Area,
    pub color: Color,
    pub direction: f64,
    pub current_time:f64
}

impl QuadData {
    pub fn new() -> QuadData { 
        let pos = Point{x: 500.0, y: 500.0};
        let area = Area{width: 300.0, height: 100.0};
        let color = Color::new(1.0, 1.0, 0.0, 1.0);
        QuadData{pos, area, color, direction: 0.05, current_time: 0.0}
    }

    pub fn update(self:&mut Self, time_stamp:f64) {
        self.current_time = time_stamp;
        let color = &mut self.color;
        let direction = &mut (self.direction);
        color.r += *direction;
        if *direction > 0.0 {
            if color.r > 1.0 {
                color.r = 1.0;
                *direction *= -1.0;
            }
        } else {
            if color.r < 0.0 {
                color.r = 0.0;
                *direction *= -1.0;
            }
        }

    }
}
