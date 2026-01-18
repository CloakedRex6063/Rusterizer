use crate::command::Command;
use crate::math::{Color, Float3, Float4, Matrix4};
use crate::viewport::Viewport;
use crate::window::Window;
use std::cmp::PartialEq;
use std::time::Instant;

mod command;
mod image_view;
mod math;
mod viewport;
mod window;

struct ImageView {
    pixels: Vec<Color>,
    width: u32,
    height: u32,
}

#[derive(Eq, PartialEq)]
enum CullMode {
    None,
    Clockwise,
    CounterClockwise,
}

struct Mesh {
    positions: Vec<Float3>,
    indices: Vec<u32>,
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
        // -X face
        Float3::new(-1.0, -1.0, -1.0),
        Float3::new(-1.0, 1.0, -1.0),
        Float3::new(-1.0, -1.0, 1.0),
        Float3::new(-1.0, 1.0, 1.0),
        // +X face
        Float3::new(1.0, -1.0, -1.0),
        Float3::new(1.0, 1.0, -1.0),
        Float3::new(1.0, -1.0, 1.0),
        Float3::new(1.0, 1.0, 1.0),
        // -Y face
        Float3::new(-1.0, -1.0, -1.0),
        Float3::new(1.0, -1.0, -1.0),
        Float3::new(-1.0, -1.0, 1.0),
        Float3::new(1.0, -1.0, 1.0),
        // +Y face
        Float3::new(-1.0, 1.0, -1.0),
        Float3::new(1.0, 1.0, -1.0),
        Float3::new(-1.0, 1.0, 1.0),
        Float3::new(1.0, 1.0, 1.0),
        // -Z face
        Float3::new(-1.0, -1.0, -1.0),
        Float3::new(1.0, -1.0, -1.0),
        Float3::new(-1.0, 1.0, -1.0),
        Float3::new(1.0, 1.0, -1.0),
        // +Z face
        Float3::new(-1.0, -1.0, 1.0),
        Float3::new(1.0, -1.0, 1.0),
        Float3::new(-1.0, 1.0, 1.0),
        Float3::new(1.0, 1.0, 1.0),
    ];

    let colors = vec![
        // -X face
        Float4::new(0.0, 1.0, 1.0, 1.0),
        Float4::new(0.0, 1.0, 1.0, 1.0),
        Float4::new(0.0, 1.0, 1.0, 1.0),
        Float4::new(0.0, 1.0, 1.0, 1.0),
        // +X face
        Float4::new(1.0, 0.0, 0.0, 1.0),
        Float4::new(1.0, 0.0, 0.0, 1.0),
        Float4::new(1.0, 0.0, 0.0, 1.0),
        Float4::new(1.0, 0.0, 0.0, 1.0),
        // -Y face
        Float4::new(1.0, 0.0, 1.0, 1.0),
        Float4::new(1.0, 0.0, 1.0, 1.0),
        Float4::new(1.0, 0.0, 1.0, 1.0),
        Float4::new(1.0, 0.0, 1.0, 1.0),
        // +Y face
        Float4::new(0.0, 1.0, 0.0, 1.0),
        Float4::new(0.0, 1.0, 0.0, 1.0),
        Float4::new(0.0, 1.0, 0.0, 1.0),
        Float4::new(0.0, 1.0, 0.0, 1.0),
        // -Z face
        Float4::new(1.0, 1.0, 0.0, 1.0),
        Float4::new(1.0, 1.0, 0.0, 1.0),
        Float4::new(1.0, 1.0, 0.0, 1.0),
        Float4::new(1.0, 1.0, 0.0, 1.0),
        // +Z face
        Float4::new(0.0, 0.0, 1.0, 1.0),
        Float4::new(0.0, 0.0, 1.0, 1.0),
        Float4::new(0.0, 0.0, 1.0, 1.0),
        Float4::new(0.0, 0.0, 1.0, 1.0),
    ];

    let indices: Vec<u32> = vec![
        // -X face
        0, 2, 1, 1, 2, 3, // +X face
        4, 5, 6, 6, 5, 7, // -Y face
        8, 9, 10, 10, 9, 11, // +Y face
        12, 14, 13, 14, 15, 13, // -Z face
        16, 18, 17, 17, 18, 19, // +Z face
        20, 21, 22, 21, 23, 22,
    ];

    let mesh = Mesh {
        positions,
        colors,
        indices,
    };

    let mut last_time = Instant::now();
    let mut time: f32 = 0.0;
    while window.is_running() {
        window.poll();

        let dt = last_time.elapsed().as_secs_f32();
        last_time = Instant::now();
        std::println!("Delta time: {dt}");
        time += dt;

        let (width, height) = window.get_window_size();
        if window.is_resized() {
            image_view = ImageView::new(width as u32, height as u32);
        }

        let viewport = Viewport {
            x_min: 0,
            y_min: 0,
            x_max: width,
            y_max: height,
        };
        command.set_viewport(viewport);

        command.set_cull_mode(CullMode::Clockwise);

        profile!("Clear Time", {
            command.clear_image(&mut image_view, Float4::new(1.0, 1.0, 1.0, 1.0));
        });

        profile!("Mesh Render Time", {
            let transform =
                    Matrix4::perspective(0.01, 10.0, std::f32::consts::PI / 3.0, width as f32 * 1.0 / height as f32)
                    * Matrix4::translate(Float3::new(0.0, 0.0, -5.0))
                    * Matrix4::scale(Float3::new(height as f32 * 1.0 / width as f32, 1.0, 1.0))
                    * Matrix4::scale_f(0.5)
                    * Matrix4::rotate_zx(time)
                    * Matrix4::rotate_xy(time);
            command.draw_mesh(&mut image_view, &mesh, transform);
        });

        profile!("Present Time", {
            window.present(&image_view);
        });
    }
}
