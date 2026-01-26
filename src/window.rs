use crate::image_view::{RenderTarget};
use sdl3::pixels::PixelFormat;
use sdl3::render::{BlendMode, Canvas, TextureCreator};
use sdl3::video::WindowContext;

struct Texture {
    pixels: Vec<u8>,
    texture: sdl3::render::Texture<'static>,
    width: u32,
    height: u32,
}

pub struct Window {
    context: sdl3::Sdl,
    video: sdl3::VideoSubsystem,
    event_pump: sdl3::EventPump,
    canvas: Canvas<sdl3::video::Window>,
    texture_creator: &'static TextureCreator<WindowContext>,
    texture: Texture,
    window_size: (i32, i32),
    mouse_pos: (i32, i32),
    running: bool,
    resized: bool,
}

impl Window {
    pub fn new(width: u32, height: u32) -> Self {
        let context = sdl3::init().unwrap();
        let video = context.video().unwrap();
        let event_pump = context.event_pump().unwrap();
        let window = video
            .window("Rusterizer", width, height)
            .resizable()
            .build()
            .unwrap();
        let canvas = window.into_canvas();
        let texture_creator: &'static _ = Box::leak(Box::new(canvas.texture_creator()));

        let mut texture_data = texture_creator
            .create_texture_streaming(PixelFormat::RGBA32, width, height)
            .unwrap();
        texture_data.set_blend_mode(BlendMode::None);

        let texture = Texture {
            pixels: vec![0; (width * height * 4) as usize],
            texture: texture_data,
            width,
            height,
        };

        Self {
            context,
            video,
            event_pump,
            canvas,
            texture_creator,
            texture,
            window_size: (width as i32, height as i32),
            mouse_pos: (0, 0),
            running: true,
            resized: false,
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn get_mouse_pos(&self) -> (i32, i32) {
        self.mouse_pos
    }

    pub fn get_window_size(&self) -> (i32, i32) {
        self.window_size
    }

    pub fn is_resized(&self) -> bool {
        self.resized
    }

    pub fn present(&mut self, render_target: &RenderTarget) {
        let src = unsafe {
            std::slice::from_raw_parts(
                render_target.pixels.as_ptr() as *const u8,
                render_target.pixels.len() * 4,
            )
        };
        self.texture.pixels.copy_from_slice(src);

        let pitch = (self.window_size.0 * 4) as usize;
        self.texture
            .texture
            .update(None, &self.texture.pixels, pitch)
            .unwrap();
        self.canvas.copy(&self.texture.texture, None, None).unwrap();
        self.canvas.present();
        self.resized = false;
    }

    pub fn poll(&mut self) {
        for event in self.event_pump.poll_iter() {
            use sdl3::event::Event;
            match event {
                Event::Quit { .. } => self.running = false,
                Event::Window {
                    win_event: sdl3::event::WindowEvent::Resized(width, height),
                    ..
                } => {
                    self.window_size = (width, height);

                    let mut texture_data = self
                        .texture_creator
                        .create_texture_streaming(PixelFormat::RGBA32, width as u32, height as u32)
                        .unwrap();
                    texture_data.set_blend_mode(BlendMode::None);

                    self.texture = Texture {
                        pixels: vec![0; (width * height * 4) as usize],
                        texture: texture_data,
                        width: width as u32,
                        height: height as u32,
                    };

                    self.resized = true;
                }
                Event::KeyDown { .. } => {}
                Event::KeyUp { .. } => {}
                Event::MouseMotion { x, y, .. } => {
                    self.mouse_pos = (x as i32, y as i32);
                }
                _ => {}
            }
        }
    }
}
