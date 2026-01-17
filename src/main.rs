use std::cmp::PartialEq;
use std::time::Instant;
use crate::math::{Float3, Float4, Color, Matrix4};
use crate::window::Window;

mod window;
mod math;

struct ImageView
{
    pixels: Vec<Color>,
    width: u32,
    height: u32,
}

#[derive(Eq, PartialEq)]
enum CullMode
{
    None,
    Clockwise,
    CounterClockwise,
}

struct Command
{
    cull_mode: CullMode,
}

impl Command
{
    fn new() -> Self
    {
        Self{
            cull_mode: CullMode::None,
        }
    }

    fn set_cull_mode(&mut self, cull_mode: CullMode)
    {
        self.cull_mode = cull_mode;
    }

    fn clear_image(&mut self, image: &mut ImageView, color: Float4)
    {
        image.clear_image(color);
    }

    fn draw_mesh(&mut self, image: &mut ImageView, mesh: &Mesh, transform: Matrix4)
    {
        let mut vertex_index: u32 = 0;
        while vertex_index + 2 < mesh.positions.len() as u32 {
            let v0 = transform * mesh.positions[vertex_index as usize].as_point();
            let mut v1 = transform * mesh.positions[vertex_index as usize + 1].as_point();
            let mut v2 = transform * mesh.positions[vertex_index as usize + 2].as_point();

            let c0 = mesh.colors[vertex_index as usize + 0];
            let c1 = mesh.colors[vertex_index as usize + 1];
            let c2 = mesh.colors[vertex_index as usize + 2];

            if self.cull_mode == CullMode::CounterClockwise
            {
                std::mem::swap(&mut v1, &mut v2);
            }

            let mut xmin: i32 = [
                v0.x.floor() as i32,
                v1.x.floor() as i32,
                v2.x.floor() as i32,
            ].iter().cloned().min().unwrap();

            xmin = xmin.max(0);

            let mut xmax: i32 = [
                v0.x.floor() as i32,
                v1.x.floor() as i32,
                v2.x.floor() as i32,
            ].iter().cloned().max().unwrap();

            xmax = xmax.min((image.width - 1) as i32);

            let mut ymin: i32 = [
                v0.y.floor() as i32,
                v1.y.floor() as i32,
                v2.y.floor() as i32,
            ].iter().cloned().min().unwrap();

            ymin = ymin.max(0);

            let mut ymax: i32 = [
                v0.y.floor() as i32,
                v1.y.floor() as i32,
                v2.y.floor() as i32,
            ].iter().cloned().max().unwrap();

            ymax = ymax.min((image.height - 1) as i32);

            for x in xmin..xmax {
                for y in ymin..ymax {
                    let p = Float4::new(x as f32 + 0.5, y as f32 + 0.5, 0.0, 0.0);
                    let det01p = math::det2d(v1 - v0, p - v0);
                    let det12p = math::det2d(v2 - v1, p - v1);
                    let det20p = math::det2d(v0 - v2, p - v2);

                    let mut det012 = math::det2d(v2 - v0, v1 - v0);
                    let ccw = det012 < 0.0;

                    match self.cull_mode {
                        CullMode::None => {
                            if ccw
                            {
                                det012 = -det012;
                            }
                        }
                        CullMode::CounterClockwise => {
                            if ccw
                            {
                                continue
                            };
                        }
                        CullMode::Clockwise => {
                            if !ccw
                            {
                                continue
                            };
                            det012 = -det012;
                        }
                    }

                    if det01p >= 0.0 && det12p >= 0.0 && det20p >= 0.0 {
                        let l0 = det01p / det012;
                        let l1 = det12p / det012;
                        let l2 = det20p / det012;
                        image.set_pixel(x as u32, y as u32, Color::from(l0 * c0 + l1 * c1 + l2 * c2));
                    }
                }
            }

            vertex_index += 3;
        }
    }
}

impl ImageView
{
    pub fn new(width: u32, height: u32) -> Self {
        ImageView
        {
            pixels: vec![Default::default(); (width * height) as usize],
            width,
            height,
        }
    }

    pub fn get_pixel(&mut self, x: u32, y: u32) -> Color
    {
        self.pixels[(y * self.width + x) as usize]
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: Color)
    {
        self.pixels[(y * self.width + x) as usize] = pixel;
    }

    pub fn get_pixels(&self) -> &Vec<Color>
    {
        &self.pixels
    }

    pub fn clear_image(&mut self, color: Float4)
    {
        let u8_color = Color::from(color);
        self.pixels.fill(u8_color);
    }
}

struct Mesh
{
    positions: Vec<Float3>,
    colors: Vec<Float4>,
}

#[macro_export]
macro_rules! profile {
    ($label:expr, { $($body:tt)* }) => {{
        let __profile_start = std::time::Instant::now();
        let __profile_result = (|| {
            $($body)*
        })();
        let __profile_elapsed = __profile_start.elapsed().as_secs_f32();
        println!("{}: {}s", $label, __profile_elapsed);
        __profile_result
    }};
}


fn main() {
    let mut window = Window::new();
    let mut image_view = ImageView::new(1280, 720);
    let mut command = Command::new();

    let positions = vec![
        Float3::new(0.0, 75.0, 0.0),
        Float3::new(50.0, 0.0, 0.0),
        Float3::new(100.0, 75.0, 0.0),
    ];
    let colors = vec![
        Float4::new(1.0, 0.0, 0.0, 0.0),
        Float4::new(0.0, 1.0, 0.0, 0.0),
        Float4::new(0.0, 0.0, 1.0, 0.0),
    ];
    let mesh = Mesh
    {
        positions,
        colors
    };
    
    let mut last_time = Instant::now();
    while window.is_running()
    {
        window.poll();

        let dt = last_time.elapsed().as_secs_f32();
        last_time = Instant::now();
        std::println!("Delta time: {dt}");

        command.set_cull_mode(CullMode::Clockwise);

        profile!("Clear Time", {
            command.clear_image(&mut image_view, Float4::new(0.0, 0.0, 0.0, 1.0));
        });

        profile!("Mesh Render Time", {
            let (mx, my) = window.get_mouse_pos();

            let mut transform = Matrix4::identity();
            for i in 0..50
            {
                transform.data[3] = mx as f32 + 100.0 * (i % 10) as f32;
                transform.data[7] = my as f32 + 100.0 * (i / 10) as f32;
                command.draw_mesh(&mut image_view, &mesh, transform);
            }

        });

        profile!("Present Time", {
            window.present(&image_view);
        });
    }
}

