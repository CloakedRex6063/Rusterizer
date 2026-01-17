use crate::{math, CullMode, ImageView, Mesh};
use crate::math::{Color, Float4, Matrix4};

pub struct Command
{
    cull_mode: CullMode,
}

impl Command
{
    pub fn new() -> Self
    {
        Self{
            cull_mode: CullMode::None,
        }
    }

    pub fn set_cull_mode(&mut self, cull_mode: CullMode)
    {
        self.cull_mode = cull_mode;
    }

    pub fn clear_image(&mut self, image: &mut ImageView, color: Float4)
    {
        image.clear_image(color);
    }

    pub fn draw_mesh(&mut self, image: &mut ImageView, mesh: &Mesh, transform: Matrix4)
    {
        let mut vertex_index: u32 = 0;
        while vertex_index + 2 < mesh.positions.len() as u32 {
            let v0 = transform * mesh.positions[vertex_index as usize].as_point();
            let mut v1 = transform * mesh.positions[vertex_index as usize + 1].as_point();
            let mut v2 = transform * mesh.positions[vertex_index as usize + 2].as_point();

            let mut det012 = math::det2d(v1 - v0, v2 - v0);
            let ccw = det012 < 0.0;

            let c0 = mesh.colors[vertex_index as usize + 0];
            let mut c1 = mesh.colors[vertex_index as usize + 1];
            let mut c2 = mesh.colors[vertex_index as usize + 2];

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
                        image.set_pixel(x as u32, y as u32, Color::from(l0 * c0 + l1 * c1 + l2 * c2));
                    }
                }
            }

            vertex_index += 3;
        }
    }
}