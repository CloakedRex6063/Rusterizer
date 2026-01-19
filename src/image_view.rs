use crate::math::{Color, Float4};

pub struct Image<T> {
    pub pixels: Vec<T>,
    pub width: u32,
    pub height: u32,
}

pub type RenderTarget = Image<Color>;
pub type DepthBuffer = Image<u32>;

impl <T: Copy + Default>Image<T> {
    pub fn new(width: u32, height: u32) -> Self {
        Image {
            pixels: vec![Default::default(); (width * height) as usize],
            width,
            height,
        }
    }

    pub fn get_pixel(&mut self, x: u32, y: u32) -> T {
        self.pixels[(y * self.width + x) as usize]
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: T) {
        self.pixels[(y * self.width + x) as usize] = pixel;
    }
}

impl RenderTarget
{
    pub fn clear_image(&mut self, color: Float4) {
        let u8_color = Color::from(color);
        self.pixels.fill(u8_color);
    }
}

impl DepthBuffer
{
    pub fn clear_image(&mut self, color: u32) {
        self.pixels.fill(color);
    }
}

#[derive(Copy, Clone, Debug)]
pub enum DepthTest
{
    Never,
    Always,
    Less,
    LessOrEqual,
    Equal,
    GreaterOrEqual,
    Greater,
    NotEqual,
}