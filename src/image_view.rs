use crate::ImageView;
use crate::math::{Color, Float4};

impl ImageView {
    pub fn new(width: u32, height: u32) -> Self {
        ImageView {
            pixels: vec![Default::default(); (width * height) as usize],
            width,
            height,
        }
    }

    pub fn get_pixel(&mut self, x: u32, y: u32) -> Color {
        self.pixels[(y * self.width + x) as usize]
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: Color) {
        self.pixels[(y * self.width + x) as usize] = pixel;
    }

    pub fn get_pixels(&self) -> &Vec<Color> {
        &self.pixels
    }

    pub fn clear_image(&mut self, color: Float4) {
        let u8_color = Color::from(color);
        self.pixels.fill(u8_color);
    }
}
