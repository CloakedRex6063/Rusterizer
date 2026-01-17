use std::cmp::PartialEq;
use std::time::Instant;
use crate::command::Command;
use crate::math::{Float3, Float4, Color, Matrix4};
use crate::window::Window;

mod window;
mod math;
mod image_view;
mod command;

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
        Float3::new(100.0, 75.0, 0.0),
        Float3::new(50.0, 0.0, 0.0),
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

        if window.is_resized()
        {
            let (width, height) = window.get_window_size();
            image_view = ImageView::new(width as u32, height as u32);
        }

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

