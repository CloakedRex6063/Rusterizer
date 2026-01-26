use crate::image_view::{DepthBuffer, DepthTest, RenderTarget, Texture};
use crate::math::{Float2, Float4, Matrix4};
use crate::meshes::Mesh;
use crate::viewport::Viewport;
use crate::{math, CullMode};

struct DepthState
{
    write: bool,
    test: DepthTest,
}

pub struct Command {
    cull_mode: CullMode,
    viewport: Viewport,
    depth_state: DepthState,
}

pub struct MeshData<'a>
{
    pub mesh: &'a Mesh,
    pub transform: Matrix4,
    pub texture: &'a Texture,
}

#[derive(Default, Debug, Clone, Copy)]
struct Vertex
{
    position: Float4,
    color: Float4,
    uv: Float2
}

impl Command {
    pub fn new() -> Self {
        Self {
            cull_mode: CullMode::None,
            viewport: Viewport {
                x_min: 0,
                y_min: 0,
                x_max: 0,
                y_max: 0,
            },
            depth_state: DepthState {
                write: false,
                test: DepthTest::Less
            },
        }
    }

    pub fn set_depth_test(&mut self, depth_test: DepthTest)
    {
        self.depth_state.test = depth_test;
    }

    pub fn toggle_depth_write(&mut self, write: bool)
    {
        self.depth_state.write = write;
    }

    pub fn set_cull_mode(&mut self, cull_mode: CullMode) {
        self.cull_mode = cull_mode;
    }

    pub fn set_viewport(&mut self, viewport: Viewport) {
        self.viewport = viewport;
    }

    pub fn clear_render_target(&mut self, image: &mut RenderTarget, color: Float4) {
        image.clear_image(color);
    }

    pub fn clear_depth_buffer(&mut self, image: &mut DepthBuffer, value: f32) {
        image.clear_image(value);
    }

    pub fn draw_mesh(&mut self, render_target: &mut RenderTarget, mut depth_buffer: Option<&mut DepthBuffer>, aspect_ratio: f32, mesh_data: MeshData) {
        for vertex_index in (0..mesh_data.mesh.indices.len() - 2).step_by(3) {
            let mut i0 = vertex_index as u32;
            let mut i1 = vertex_index as u32 + 1;
            let mut i2 = vertex_index as u32 + 2;

            if !mesh_data.mesh.indices.is_empty() {
                i0 = mesh_data.mesh.indices[i0 as usize];
                i1 = mesh_data.mesh.indices[i1 as usize];
                i2 = mesh_data.mesh.indices[i2 as usize];
            }

            let perspective = Matrix4::perspective(
                0.01,
                100.0,
                std::f32::consts::PI / 3.0,
                aspect_ratio,
            );

            let mut vertices = [Vertex { position: Float4::zero(), color: Float4::zero(), uv: Float2::zero() }; 3];
            vertices[0].position = perspective * mesh_data.transform * mesh_data.mesh.positions[i0 as usize].as_point();
            vertices[1].position = perspective * mesh_data.transform * mesh_data.mesh.positions[i1 as usize].as_point();
            vertices[2].position = perspective * mesh_data.transform * mesh_data.mesh.positions[i2 as usize].as_point();
            vertices[0].uv = mesh_data.mesh.uvs[i0 as usize];
            vertices[1].uv = mesh_data.mesh.uvs[i1 as usize];
            vertices[2].uv = mesh_data.mesh.uvs[i2 as usize];

            let (clipped_vertices, count) = clip_vertices(vertices);

            for triangle in clipped_vertices[..count as usize].chunks_exact(3)  {
                let mut v0 = triangle[0].position;
                let mut v1 = triangle[1].position;
                let mut v2 = triangle[2].position;

                let uv0 = triangle[0].uv;
                let mut uv1 = triangle[1].uv;
                let mut uv2 = triangle[2].uv;

                v0 = math::perspective_divide(v0);
                v1 = math::perspective_divide(v1);
                v2 = math::perspective_divide(v2);

                v0 = self.viewport.to_screen_space(v0);
                v1 = self.viewport.to_screen_space(v1);
                v2 = self.viewport.to_screen_space(v2);

                let mut det012 = (v1 - v0).det2d(v2 - v0);
                let ccw = det012 < 0.0;

                match self.cull_mode {
                    CullMode::None => {
                        if ccw {
                            std::mem::swap(&mut v1, &mut v2);
                            std::mem::swap(&mut uv1, &mut uv2);
                            det012 = -det012;
                        }
                    }
                    CullMode::Clockwise => {
                        if !ccw {
                            continue;
                        }
                        std::mem::swap(&mut v1, &mut v2);
                        std::mem::swap(&mut uv1, &mut uv2);
                        det012 = -det012;
                    }
                    CullMode::CounterClockwise => {
                        if ccw {
                            continue;
                        }
                    }
                }

                let mut x_min: i32 = self.viewport.x_min.max(0);
                let mut x_max: i32 = self.viewport.x_max.min(render_target.width as i32) - 1;
                let mut y_min: i32 = self.viewport.y_min.max(0);
                let mut y_max: i32 = self.viewport.y_max.min(render_target.height as i32) - 1;

                let tri_x_min = v0.x.floor().min(v1.x.floor()).min(v2.x.floor()) as i32;

                let tri_x_max = v0.x.floor().max(v1.x.floor()).max(v2.x.floor()) as i32;

                let tri_y_min = v0.y.floor().min(v1.y.floor()).min(v2.y.floor()) as i32;

                let tri_y_max = v0.y.floor().max(v1.y.floor()).max(v2.y.floor()) as i32;

                x_min = x_min.max(tri_x_min);
                x_max = x_max.min(tri_x_max);
                y_min = y_min.max(tri_y_min);
                y_max = y_max.min(tri_y_max);

                for x in x_min..=x_max {
                    for y in y_min..=y_max {
                        let p = Float4::new(x as f32 + 0.5, y as f32 + 0.5, 0.0, 0.0);
                        let det01p = (v1 - v0).det2d(p - v0);
                        let det12p = (v2 - v1).det2d(p - v1);
                        let det20p = (v0 - v2).det2d(p - v2);


                        if det01p >= 0.0 && det12p >= 0.0 && det20p >= 0.0 {
                            let mut l0 = (v1 - p).det2d(v2 - p) / det012 / v0.w;
                            let mut l1 = (v2 - p).det2d(v0 - p) / det012 / v1.w;
                            let mut l2 = (v0 - p).det2d(v1 - p) / det012 / v2.w;

                            let l_sum = l0 + l1 + l2;

                            l0 /= l_sum;
                            l1 /= l_sum;
                            l2 /= l_sum;

                            let tex_coords = l0 * uv0 + l1 * uv1 + l2 * uv2;

                            let color = mesh_data.texture.pixel_at_uv(tex_coords);

                            if let Some(depth_buffer) = depth_buffer.as_mut() {
                                let old_depth = depth_buffer.get_pixel(x as u32, y as u32);

                                let z = l0 * v0.z + l1 * v1.z + l2 * v2.z;

                                if passed_depth_test(self.depth_state.test, z, old_depth) {
                                    if self.depth_state.write {
                                        depth_buffer.set_pixel(x as u32, y as u32, z);
                                    }
                                }
                                else {
                                    continue;
                                }
                            }
                            render_target.set_pixel(x as u32, y as u32, color);
                        }
                    }
                }
            }
        }

    }
}

fn clip_intersect_edge(v0: Vertex, v1: Vertex, val0: f32, val1: f32) -> Vertex {
    let t = val0 / (val0 - val1);
    Vertex{
        position: (1.0 - t) * v0.position + t * v1.position,
        color: (1.0 - t) * v0.color + t * v1.color,
        uv: (1.0 - t) * v0.uv + t * v1.uv,
    }
}

fn clip_triangle_against_plane(vertices: &[Vertex], equation: Float4, clipped: &mut [Vertex; 12], count: &mut u32) {
    let values = [
        vertices[0].position.dot(equation),
        vertices[1].position.dot(equation),
        vertices[2].position.dot(equation),
    ];

    let mask: u8 =
        (values[0] < 0.0) as u8
            | ((values[1] < 0.0) as u8) << 1
            | ((values[2] < 0.0) as u8) << 2;

    match mask {
        0b000 => {
            clipped[*count as usize] = vertices[0];
            *count += 1;
            clipped[*count as usize] = vertices[1];
            *count += 1;
            clipped[*count as usize] = vertices[2];
            *count += 1;
        }
        0b001 => {
            let v01 = clip_intersect_edge(vertices[0], vertices[1], values[0], values[1]);
            let v02 = clip_intersect_edge(vertices[0], vertices[2], values[0], values[2]);
            clipped[*count as usize] = v01;
            *count += 1;
            clipped[*count as usize] = vertices[1];
            *count += 1;
            clipped[*count as usize] = vertices[2];
            *count += 1;
            clipped[*count as usize] = v01;
            *count += 1;
            clipped[*count as usize] = vertices[2];
            *count += 1;
            clipped[*count as usize] = v02;
            *count += 1;
        }
        0b010 => {
            let v10 = clip_intersect_edge(vertices[1], vertices[0], values[1], values[0]);
            let v12 = clip_intersect_edge(vertices[1], vertices[2], values[1], values[2]);
            clipped[*count as usize] = vertices[0];
            *count += 1;
            clipped[*count as usize] = v10;
            *count += 1;
            clipped[*count as usize] = vertices[2];
            *count += 1;
            clipped[*count as usize] = vertices[2];
            *count += 1;
            clipped[*count as usize] = v10;
            *count += 1;
            clipped[*count as usize] = v12;
            *count += 1;
        }
        0b011 => {
            let v02 = clip_intersect_edge(vertices[0], vertices[2], values[0], values[2]);
            let v12 = clip_intersect_edge(vertices[1], vertices[2], values[1], values[2]);
            clipped[*count as usize] = v02;
            *count += 1;
            clipped[*count as usize] = v12;
            *count += 1;
            clipped[*count as usize] = vertices[2];
            *count += 1;
        }
        0b100 => {
            let v20 = clip_intersect_edge(vertices[2], vertices[0], values[2], values[0]);
            let v21 = clip_intersect_edge(vertices[2], vertices[1], values[2], values[1]);
            clipped[*count as usize] = vertices[0];
            *count += 1;
            clipped[*count as usize] = vertices[1];
            *count += 1;
            clipped[*count as usize] = v20;
            *count += 1;
            clipped[*count as usize] = v20;
            *count += 1;
            clipped[*count as usize] = vertices[1];
            *count += 1;
            clipped[*count as usize] = v21;
            *count += 1;
        }
        0b101 => {
            let v01 = clip_intersect_edge(vertices[0], vertices[1], values[0], values[1]);
            let v21 = clip_intersect_edge(vertices[2], vertices[1], values[2], values[1]);
            clipped[*count as usize] = v01;
            *count += 1;
            clipped[*count as usize] = vertices[1];
            *count += 1;
            clipped[*count as usize] = v21;
            *count += 1;
        }
        0b110 => {
            let v10 = clip_intersect_edge(vertices[1], vertices[0], values[1], values[0]);
            let v20 = clip_intersect_edge(vertices[2], vertices[0], values[2], values[0]);
            clipped[*count as usize] = vertices[0];
            *count += 1;
            clipped[*count as usize] = v10;
            *count += 1;
            clipped[*count as usize] = v20;
            *count += 1;
        }
        0b111 => {

        }
        _ => {},
    }
}

fn clip_vertices(vertices: [Vertex; 3]) -> ([Vertex; 12], u32) {
    let mut input: [Vertex; 12] = [Vertex::default(); 12];
    input[0..3].copy_from_slice(&vertices);
    let mut count = 3u32;
    let equations: [Float4; 2] = [
        Float4::new(0.0, 0.0, 1.0, 0.0),
        Float4::new(0.0, 0.0, -1.0, 1.0),
    ];

    for equation in equations.into_iter() {
        let mut output: [Vertex; 12] = [Vertex::default(); 12];
        for triangle in input[0..count as usize].chunks_exact(3) {
            clip_triangle_against_plane(triangle, equation, &mut output, &mut count);
        }
        input = output;
    }

    (input, count)
}

const fn passed_depth_test(depth_test: DepthTest, value: f32, reference: f32) -> bool {
    match depth_test {
        DepthTest::Never => false,
        DepthTest::Always => true,
        DepthTest::Less => value < reference,
        DepthTest::LessOrEqual => value <= reference,
        DepthTest::Equal => value == reference,
        DepthTest::GreaterOrEqual => value >= reference,
        DepthTest::Greater => value > reference,
        DepthTest::NotEqual => value != reference,
    }
}