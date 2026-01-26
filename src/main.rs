use crate::command::{Command, Shader};
use crate::image_view::{DepthBuffer, DepthTest, RenderTarget, Texture};
use crate::math::Interpolate;
use crate::math::{Float2, Float3, Float4, Matrix4};
use crate::meshes::{Cube, Mesh};
use crate::viewport::Viewport;
use crate::window::Window;
use interpolate_macro::Interpolate;
use std::cmp::PartialEq;
use std::path::Path;
use std::time::Instant;

mod command;
mod image_view;
mod light;
mod math;
mod meshes;
mod viewport;
mod window;

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

    pub struct MeshData<'a> {
        pub mesh: &'a Mesh,
        pub model: Matrix4,
        pub perspective: Matrix4,
        pub texture: &'a Texture,
    }

    #[derive(Default, Debug, Clone, Copy, Interpolate)]
    struct VertexOutput {
        pub position: Float4,
        pub world_pos: Float4,
        pub uv: Float2,
    }

    let shader = Shader {
        vertex_shader: Box::new(
            |vertex_index, mesh_data: &MeshData| -> (VertexOutput, Float4) {
                let mut vertex = VertexOutput {
                    position: Float4::zero(),
                    world_pos: Float4::zero(),
                    uv: Float2::zero(),
                };
                vertex.position = mesh_data.perspective
                    * mesh_data.model
                    * mesh_data.mesh.positions[vertex_index as usize].as_point();
                vertex.world_pos =
                    mesh_data.model * mesh_data.mesh.positions[vertex_index as usize].as_point();
                vertex.uv = mesh_data.mesh.uvs[vertex_index as usize];
                (vertex, vertex.position)
            },
        ),
        fragment_shader: Box::new(|vertex: &VertexOutput, fragment_input: &MeshData| {
            let color = fragment_input.texture.pixel_at_uv(vertex.uv);
            color
        }),
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

        let perspective =
            Matrix4::perspective(0.01, 100.0, std::f32::consts::PI / 3.0, aspect_ratio);

        profile!("Mesh Render Time", {
            command.set_positions(&cube.mesh.positions);
            command.set_indices(&cube.mesh.indices);
            let model = Matrix4::translate(Float3::new(0.0, 1.0, -5.0))
                * Matrix4::rotate_yz(time)
                * Matrix4::rotate_xy(time);

            let mesh_data = MeshData {
                mesh: &cube.mesh,
                model,
                perspective,
                texture: &texture,
            };

            command.draw_indexed(
                &mut render_target,
                &mut depth_buffer,
                &shader,
                &mesh_data,
                &mesh_data,
            );
        });

        profile!("Present Time", {
            window.present(&render_target);
        });
    }
}
