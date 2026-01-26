use crate::math::{Color, Float2, Float4};
use std::path::Path;

pub struct Image<T> {
    pub pixels: Vec<T>,
    pub width: u32,
    pub height: u32,
}

pub type Texture = Image<Color>;
pub type RenderTarget = Image<Color>;
pub type DepthBuffer = Image<f32>;

impl<T: Copy + Default> Image<T> {
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

impl RenderTarget {
    pub fn clear_image(&mut self, color: Float4) {
        let u8_color = Color::from(color);
        self.pixels.fill(u8_color);
    }
}

impl DepthBuffer {
    pub fn clear_image(&mut self, color: f32) {
        self.pixels.fill(color);
    }
}

impl Texture {
    pub fn from_file(path: &Path) -> Texture {
        let decoded_image = stb_image::image::load(path);
        if let stb_image::image::LoadResult::ImageU8(image) = decoded_image {
            let mut texture = Texture::new(image.width as u32, image.height as u32);
            for y in 0..image.height {
                for x in 0..image.width {
                    let index = (y * image.width + x) * 3;
                    texture.set_pixel(
                        x as u32,
                        y as u32,
                        Color::new(
                            image.data[index],
                            image.data[index + 1],
                            image.data[index + 2],
                        ),
                    );
                }
            }
            texture
        } else {
            panic!("Unsupported texture type");
        }
    }

    pub fn pixel_at_uv(&self, uv: Float2) -> Color
    {
        let mut x = (uv.x * self.width as f32) as usize;
        let mut y = (uv.y * self.height as f32) as usize;
        x = x.clamp(0, self.width as usize - 1);
        y = y.clamp(0, self.height as usize - 1);
        let id = y * self.width as usize + x;
        self.pixels[id]
    }
}

#[derive(Copy, Clone, Debug)]
pub enum DepthTest {
    Never,
    Always,
    Less,
    LessOrEqual,
    Equal,
    GreaterOrEqual,
    Greater,
    NotEqual,
}
