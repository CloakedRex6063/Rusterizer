use num_traits::{One, Zero};
use std::ops;

pub trait Number: Copy + Zero + One {}

impl Number for f32 {}
impl Number for f64 {}
impl Number for i32 {}
impl Number for i64 {}
impl Number for u32 {}
impl Number for u64 {}

#[derive(Default, Copy, Clone, Debug)]
pub struct Vec2<T: Number> {
    pub x: T,
    pub y: T,
}
impl<T: Number> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

pub type UInt2 = Vec2<u32>;
pub type Int2 = Vec2<i32>;
pub type Float2 = Vec2<f32>;

impl<T: Number + ops::Add<Output = T>> ops::Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn add(self, _rhs: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl<T: Number + ops::Sub<Output = T>> ops::Sub<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn sub(self, _rhs: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

impl<T: Number + ops::Mul<Output = T>> ops::Mul<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn mul(self, _rhs: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
        }
    }
}

impl<T: Number + ops::Div<Output = T>> ops::Div<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn div(self, _rhs: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x / _rhs.x,
            y: self.y / _rhs.y,
        }
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Vec3<T: Number> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Number> Vec3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn as_point(&self) -> Vec4<T> {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: T::one(),
        }
    }

    pub fn as_vector(&self) -> Vec4<T> {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: T::zero(),
        }
    }
}

pub type UInt3 = Vec3<u32>;
pub type Int3 = Vec3<i32>;
pub type Float3 = Vec3<f32>;

impl<T: Number + ops::Add<Output = T>> ops::Add<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn add(self, _rhs: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl<T: Number + ops::Sub<Output = T>> ops::Sub<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn sub(self, _rhs: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl<T: Number + ops::Mul<Output = T>> ops::Mul<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn mul(self, _rhs: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
        }
    }
}

impl<T: Number + ops::Div<Output = T>> ops::Div<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn div(self, _rhs: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x / _rhs.x,
            y: self.y / _rhs.y,
            z: self.z / _rhs.z,
        }
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Vec4<T: Number> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}
impl<T: Number> Vec4<T> {
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}

pub type UInt4 = Vec4<u32>;
pub type Int4 = Vec4<i32>;
pub type Float4 = Vec4<f32>;

impl<T: Number + ops::Add<Output = T>> ops::Add<Vec4<T>> for Vec4<T> {
    type Output = Vec4<T>;
    fn add(self, _rhs: Vec4<T>) -> Vec4<T> {
        Vec4 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
            w: self.w + _rhs.w,
        }
    }
}

impl<T: Number + ops::Sub<Output = T>> ops::Sub<Vec4<T>> for Vec4<T> {
    type Output = Vec4<T>;
    fn sub(self, _rhs: Vec4<T>) -> Vec4<T> {
        Vec4 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
            w: self.w - _rhs.w,
        }
    }
}

impl<T: Number + ops::Mul<Output = T>> ops::Mul<Vec4<T>> for Vec4<T> {
    type Output = Vec4<T>;
    fn mul(self, _rhs: Vec4<T>) -> Vec4<T> {
        Vec4 {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
            w: self.w * _rhs.w,
        }
    }
}

impl<T: Number + ops::Div<Output = T>> ops::Div<Vec4<T>> for Vec4<T> {
    type Output = Vec4<T>;
    fn div(self, _rhs: Vec4<T>) -> Vec4<T> {
        Vec4 {
            x: self.x / _rhs.x,
            y: self.y / _rhs.y,
            z: self.z / _rhs.z,
            w: self.w / _rhs.w,
        }
    }
}

pub fn det2d(v0: Float4, v1: Float4) -> f32 {
    v0.x * v1.y - v0.y * v1.x
}

#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub struct Color {
    color: [u8; 4],
}

impl From<Float4> for Color {
    fn from(c: Float4) -> Self {
        Self {
            color: [
                (c.x * 255.0).clamp(0.0, 255.0) as u8,
                (c.y * 255.0).clamp(0.0, 255.0) as u8,
                (c.z * 255.0).clamp(0.0, 255.0) as u8,
                (c.w * 255.0).clamp(0.0, 255.0) as u8,
            ],
        }
    }
}

impl From<Color> for Float4 {
    fn from(c: Color) -> Self {
        Self {
            x: c.color[0] as f32 / 255.0,
            y: c.color[1] as f32 / 255.0,
            z: c.color[2] as f32 / 255.0,
            w: c.color[3] as f32 / 255.0,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Matrix4 {
    pub data: [f32; 16],
}

impl Matrix4 {
    pub fn new() -> Self {
        Self { data: [0.0; 16] }
    }

    pub fn identity() -> Self {
        Self {
            data: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
}

impl ops::Mul<Float4> for Matrix4 {
    type Output = Float4;
    fn mul(self, rhs: Float4) -> Float4 {
        Float4::new(
            self.data[0] * rhs.x
                + self.data[1] * rhs.y
                + self.data[2] * rhs.z
                + self.data[3] * rhs.w,
            self.data[4] * rhs.x
                + self.data[5] * rhs.y
                + self.data[6] * rhs.z
                + self.data[7] * rhs.w,
            self.data[8] * rhs.x
                + self.data[9] * rhs.y
                + self.data[10] * rhs.z
                + self.data[11] * rhs.w,
            self.data[12] * rhs.x
                + self.data[13] * rhs.y
                + self.data[14] * rhs.z
                + self.data[15] * rhs.w,
        )
    }
}

impl ops::Mul<Matrix4> for Float4 {
    type Output = Float4;

    fn mul(self, rhs: Matrix4) -> Float4 {
        Float4::new(
            self.x * rhs.data[0]
                + self.y * rhs.data[4]
                + self.z * rhs.data[8]
                + self.w * rhs.data[12],
            self.x * rhs.data[1]
                + self.y * rhs.data[5]
                + self.z * rhs.data[9]
                + self.w * rhs.data[13],
            self.x * rhs.data[2]
                + self.y * rhs.data[6]
                + self.z * rhs.data[10]
                + self.w * rhs.data[14],
            self.x * rhs.data[3]
                + self.y * rhs.data[7]
                + self.z * rhs.data[11]
                + self.w * rhs.data[15],
        )
    }
}

impl ops::Mul<Float4> for f32 {
    type Output = Float4;

    fn mul(self, rhs: Float4) -> Float4 {
        Float4::new(self * rhs.x, self * rhs.y, self * rhs.z, self * rhs.w)
    }
}

impl ops::Mul<f32> for Float4 {
    type Output = Float4;

    fn mul(self, rhs: f32) -> Float4 {
        Float4::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}
