use crate::math::{Color, Float4, Matrix4};
use crate::viewport::Viewport;
use crate::{CullMode, ImageView, Mesh, math};

pub struct Command {
    cull_mode: CullMode,
    viewport: Viewport,
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
        }
    }

    pub fn set_cull_mode(&mut self, cull_mode: CullMode) {
        self.cull_mode = cull_mode;
    }

    pub fn set_viewport(&mut self, viewport: Viewport) {
        self.viewport = viewport;
    }

    pub fn clear_image(&mut self, image: &mut ImageView, color: Float4) {
        image.clear_image(color);
    }

    pub fn draw_mesh(&mut self, image: &mut ImageView, mesh: &Mesh, transform: Matrix4) {
        for vertex_index in (0usize..mesh.positions.len())
            .step_by(3)
            .take_while(|&i| i + 2 < mesh.positions.len())
        {
            let mut v0 = transform * mesh.positions[vertex_index].as_point();
            let mut v1 = transform * mesh.positions[vertex_index + 1].as_point();
            let mut v2 = transform * mesh.positions[vertex_index + 2].as_point();

            v0 = self.viewport.to_screen_space(v0);
            v1 = self.viewport.to_screen_space(v1);
            v2 = self.viewport.to_screen_space(v2);

            let mut det012 = math::det2d(v1 - v0, v2 - v0);
            let ccw = det012 < 0.0;

            let c0 = mesh.colors[vertex_index + 0];
            let mut c1 = mesh.colors[vertex_index + 1];
            let mut c2 = mesh.colors[vertex_index + 2];

            match self.cull_mode {
                CullMode::None => {
                    if !ccw {
                        std::mem::swap(&mut v1, &mut v2);
                        std::mem::swap(&mut c1, &mut c2);
                        det012 = -det012;
                    }
                }
                CullMode::Clockwise => {
                    if !ccw {
                        continue;
                    }
                    std::mem::swap(&mut v1, &mut v2);
                    std::mem::swap(&mut c1, &mut c2);
                    det012 = -det012;
                }
                CullMode::CounterClockwise => {
                    if ccw {
                        continue;
                    }
                }
            }

            let mut xmin: i32 = self.viewport.x_min.max(0);
            let mut xmax: i32 = self.viewport.x_max.min(image.width as i32) - 1;
            let mut ymin: i32 = self.viewport.y_min.max(0);
            let mut ymax: i32 = self.viewport.y_max.min(image.height as i32) - 1;

            let tri_x_min = v0.x.floor().min(v1.x.floor()).min(v2.x.floor()) as i32;

            let tri_x_max = v0.x.floor().max(v1.x.floor()).max(v2.x.floor()) as i32;

            let tri_y_min = v0.y.floor().min(v1.y.floor()).min(v2.y.floor()) as i32;

            let tri_y_max = v0.y.floor().max(v1.y.floor()).max(v2.y.floor()) as i32;

            xmin = xmin.max(tri_x_min);
            xmax = xmax.min(tri_x_max);
            ymin = ymin.max(tri_y_min);
            ymax = ymax.min(tri_y_max);

            for x in xmin..=xmax {
                for y in ymin..=ymax {
                    let p = Float4::new(x as f32 + 0.5, y as f32 + 0.5, 0.0, 0.0);
                    let det01p = math::det2d(v1 - v0, p - v0);
                    let det12p = math::det2d(v2 - v1, p - v1);
                    let det20p = math::det2d(v0 - v2, p - v2);

                    if det01p >= 0.0 && det12p >= 0.0 && det20p >= 0.0 {
                        let l0 = det01p / det012;
                        let l1 = det12p / det012;
                        let l2 = det20p / det012;
                        image.set_pixel(
                            x as u32,
                            y as u32,
                            Color::from(l0 * c0 + l1 * c1 + l2 * c2),
                        );
                    }
                }
            }
        }
    }
}
