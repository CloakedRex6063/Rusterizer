use crate::command::{Command, MeshData};
use crate::image_view::{DepthBuffer, DepthTest, Texture, RenderTarget};
use crate::math::{Float3, Float4, Matrix4};
use crate::meshes::Cube;
use crate::viewport::Viewport;
use crate::window::Window;
use std::cmp::PartialEq;
use std::path::Path;
use std::time::Instant;

mod command;
mod image_view;
mod math;
mod meshes;
mod viewport;
mod window;
mod light;

#[derive(Eq, PartialEq)]
enum CullMode {
    None,
    Clockwise,
    CounterClockwise,
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
    let mut window = Window::new(1280, 720);
    let mut render_target = RenderTarget::new(1280, 720);

    let texture = Texture::from_file(Path::new("assets/bojan.jpg"));
    let mut depth_buffer = DepthBuffer::new(1280, 720);
    let mut command = Command::new();

    let cube = Cube::new();

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
            render_target = RenderTarget::new(width as u32, height as u32);
            depth_buffer = DepthBuffer::new(width as u32, height as u32);
        }

        let viewport = Viewport {
            x_min: 0,
            y_min: 0,
            x_max: width,
            y_max: height,
        };
        command.set_viewport(viewport);

        command.set_cull_mode(CullMode::Clockwise);

        command.set_depth_test(DepthTest::Less);
        command.toggle_depth_write(true);

        profile!("Clear Time", {
            command.clear_render_target(&mut render_target, Float4::new(1.0, 1.0, 1.0, 1.0));
            command.clear_depth_buffer(&mut depth_buffer, 1.0);
        });

        let aspect_ratio = width as f32 / height as f32;
        profile!("Mesh Render Time", {
            for i in -0..1 {
                let model = Matrix4::translate(Float3::new(i as f32 * 1.5, 0.0, -5.0))
                    * Matrix4::rotate_yz(time)
                    * Matrix4::rotate_xy(time);
                command.draw_mesh(
                    &mut render_target,
                    Some(&mut depth_buffer),
                    aspect_ratio,
                    MeshData {
                        mesh: &cube.mesh,
                        transform: model,
                        texture: &texture,
                    },
                );
            }
        });

        profile!("Present Time", {
            window.present(&render_target);
        });
    }
}
