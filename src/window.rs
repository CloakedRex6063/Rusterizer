use sdl3::pixels::PixelFormat;
use sdl3::render::{BlendMode, Canvas, TextureCreator};
use sdl3::video::WindowContext;
use crate::{ImageView};

struct Texture
{
    pixels: Vec<u8>,
    texture: sdl3::render::Texture<'static>,
    width: u32,
    height: u32,
}

pub struct Window
{
    context: sdl3::Sdl,
    video: sdl3::VideoSubsystem,
    event_pump: sdl3::EventPump,
    canvas: Canvas<sdl3::video::Window>,
    texture_creator: &'static TextureCreator<WindowContext>,
    texture: Texture,
    window_size: (i32, i32),
    mouse_pos: (i32, i32),
    running: bool,
}

impl Window
{
    pub fn new() -> Self {
        let context =  sdl3::init().unwrap();
        let video = context.video().unwrap();
        let event_pump = context.event_pump().unwrap();
        let window = video.window("Rusterizer", 1280, 720).resizable().build().unwrap();
        let canvas = window.into_canvas();
        let texture_creator: &'static _ = Box::leak(Box::new(canvas.texture_creator()));

        let mut texture_data = texture_creator
            .create_texture_streaming(PixelFormat::RGBA32, 1280, 720)
            .unwrap();
        texture_data.set_blend_mode(BlendMode::None);

        let texture = Texture {
            pixels: vec![0; 1280 * 720 * 4],
            texture: texture_data,
            width: 1280,
            height: 720,
        };

        Self{
            context,
            video,
            event_pump,
            canvas,
            texture_creator,
            texture,
            window_size: (1280, 720),
            mouse_pos: (0, 0),
            running: true
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn get_mouse_pos(&self) -> (i32, i32)
    {
        self.mouse_pos
    }

    pub fn get_window_size(&self) -> (i32, i32)
    {
        self.window_size
    }

    pub fn present(&mut self, image_view: &ImageView)
    {
        let src = unsafe {
            std::slice::from_raw_parts(
                image_view.get_pixels().as_ptr() as *const u8,
                image_view.get_pixels().len() * 4,
            )
        };
        self.texture.pixels[..src.len()].copy_from_slice(src);

        let pitch = (self.window_size.0 * 4) as usize;
        self.texture.texture.update(None, &self.texture.pixels, pitch).unwrap();
        self.canvas.copy(&self.texture.texture, None, None).unwrap();
        self.canvas.present();
    }

    pub fn poll(&mut self)
    {
        for event in self.event_pump.poll_iter() {
            use sdl3::event::Event;
            match event{
                Event::Quit { .. } => { self.running = false }
                Event::Window { win_event: sdl3::event::WindowEvent::Resized(width, height), .. } => {
                    //TODO: Resize Texture
                    self.window_size = (width, height);
                }
                Event::KeyDown { .. } => {}
                Event::KeyUp { .. } => {}
                Event::MouseMotion {
                    x,
                    y,
                    ..
                } => {
                    self.mouse_pos = (x as i32, y as i32);
                }
                _ => {}
            }
        }
    }
}