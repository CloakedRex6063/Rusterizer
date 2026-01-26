use crate::image_view::{DepthBuffer, DepthTest, RenderTarget};
use crate::math::{Color, Float4, Interpolate};
use crate::viewport::Viewport;
use crate::{math, CullMode};

struct DepthState
{
    write: bool,
    test: DepthTest,
}

pub struct Command<'a> {
    cull_mode: CullMode,
    viewport: Viewport,
    depth_state: DepthState,
    indices: Option<&'a [u32]>,
}

pub struct Shader<VertexInput, VertexOutput, FragmentInput>
{
    pub vertex_shader: Box<dyn Fn(u32, &VertexInput) -> ([VertexOutput; 3], [Float4; 3])>,
    pub fragment_shader: Box<dyn Fn(&VertexOutput, &FragmentInput) -> Color>,
}

impl <'a>Command<'a> {
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
            indices: None,
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

    pub fn set_indices(&mut self, indices: &'a [u32])
    {
        self.indices = Some(indices);
    }

    pub fn clear_render_target(&mut self, image: &mut RenderTarget, color: Float4) {
        image.clear_image(color);
    }

    pub fn clear_depth_buffer(&mut self, image: &mut DepthBuffer, value: f32) {
        image.clear_image(value);
    }

    pub fn draw_mesh<VertexInput, VertexOutput , FragmentInput>(&mut self, render_target: &mut RenderTarget, mut depth_buffer: Option<&mut DepthBuffer>, shader: &Shader<VertexInput, VertexOutput, FragmentInput>, vertex_input: &VertexInput, fragment_input: &FragmentInput)
    where VertexOutput: Interpolate, {
        for vertex_index in (0..self.indices.unwrap().len() - 2).step_by(3) {

            let (mut vertex_output, positions) = (shader.vertex_shader)(vertex_index as u32, &vertex_input);

            let (clipped_vertices, count) = clip_vertices(positions);

            for triangle in clipped_vertices[..count as usize].chunks_exact(3)  {
                let mut v0 = triangle[0];
                let mut v1 = triangle[1];
                let mut v2 = triangle[2];

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
                            vertex_output.swap(2, 1);
                            det012 = -det012;
                        }
                    }
                    CullMode::Clockwise => {
                        if !ccw {
                            continue;
                        }
                        std::mem::swap(&mut v1, &mut v2);
                        vertex_output.swap(2, 1);
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

                            let interpolated_output = VertexOutput::interp(l0, l1, l2, &vertex_output[0], &vertex_output[1], &vertex_output[2]);


                            let color = (shader.fragment_shader)(&interpolated_output, &fragment_input);

                            render_target.set_pixel(x as u32, y as u32, color);
                        }
                    }
                }
            }
        }

    }
}

fn clip_intersect_edge(v0: Float4, v1: Float4, val0: f32, val1: f32) -> Float4 {
    let t = val0 / (val0 - val1);
    (1.0 - t) * v0 + t * v1
}

fn clip_triangle_against_plane(vertices: &[Float4], equation: Float4, clipped: &mut [Float4; 12], count: &mut u32) {
    let values = [
        vertices[0].dot(equation),
        vertices[1].dot(equation),
        vertices[2].dot(equation),
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

fn clip_vertices(vertices: [Float4; 3]) -> ([Float4; 12], u32) {
    let mut input: [Float4; 12] = [Float4::default(); 12];
    input[0..3].copy_from_slice(&vertices);
    let mut count = 3u32;
    let equations: [Float4; 2] = [
        Float4::new(0.0, 0.0, 1.0, 0.0),
        Float4::new(0.0, 0.0, -1.0, 1.0),
    ];

    for equation in equations.into_iter() {
        let mut output: [Float4; 12] = [Float4::default(); 12];
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