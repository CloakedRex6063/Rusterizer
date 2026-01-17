use crate::math::Float4;

pub struct Viewport {
    pub x_min: i32,
    pub y_min: i32,
    pub x_max: i32,
    pub y_max: i32,
}

impl Viewport {
    pub fn to_screen_space(&self, mut v: Float4) -> Float4 {
        let width = (self.x_max - self.x_min) as f32;
        let height = (self.y_max - self.y_min) as f32;

        v.x = self.x_min as f32 + (v.x * 0.5 + 0.5) * width;
        v.y = self.y_min as f32 + (1.0 - (v.y * 0.5 + 0.5)) * height;

        v
    }
}
