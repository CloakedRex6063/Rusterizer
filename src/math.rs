use num_traits::{One, Zero};
use std::ops;
use std::ops::{AddAssign, Mul};

pub trait Number: Copy + Zero + One {}

pub trait Interpolate: Sized {
    fn interp(l0: f32, l1: f32, l2: f32, a: &Self, b: &Self, c: &Self) -> Self;
}

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

    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }

    pub fn dot(self, _rhs: Vec4<T>) -> T {
        self.x * _rhs.x + self.y * _rhs.y
    }
}

pub type UInt2 = Vec2<u32>;
pub type Int2 = Vec2<i32>;
pub type Float2 = Vec2<f32>;

impl Interpolate for Float2
{
    fn interp(l0: f32, l1: f32, l2: f32, a: &Self, b: &Self, c: &Self) -> Self
    {
        l0 * *a + l1 * *b + l2 * *c
    }
}

impl Mul<Float2> for f32 {
    type Output = Float2;

    fn mul(self, rhs: Float2) -> Self::Output {
        Float2::new(self * rhs.x, self * rhs.y)
    }
}

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

    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero(), T::zero())
    }

    pub fn dot(self, _rhs: Vec3<T>) -> T {
        self.x * _rhs.x + self.y * _rhs.y + self.z * _rhs.z
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

impl Float3
{
    pub fn normalize(self) -> Self {
        let len = self.length();
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn length_squared(&self) -> f32
    {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f32
    {
        self.length_squared().sqrt()
    }
}

impl Mul<Float3> for f32 {
    type Output = Float3;

    fn mul(self, rhs: Float3) -> Self::Output {
        Float3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Interpolate for Float3
{
    fn interp(l0: f32, l1: f32, l2: f32, a: &Self, b: &Self, c: &Self) -> Self
    {
        l0 * *a + l1 * *b + l2 * *c
    }
}


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

    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero(), T::zero(), T::zero())
    }

    pub fn dot(self, _rhs: Vec4<T>) -> T {
        self.x * _rhs.x + self.y * _rhs.y + self.z * _rhs.z + self.w * _rhs.w
    }
}

impl Float4
{
    pub fn det2d(self, v1: Float4) -> f32 {
        self.x * v1.y - self.y * v1.x
    }
}

impl Interpolate for Float4
{
    fn interp(l0: f32, l1: f32, l2: f32, a: &Self, b: &Self, c: &Self) -> Self
    {
        *a * l0 + *b * l1 + *c * l2
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
#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub struct Color {
    color: [u8; 4],
}

impl ops::Index<usize> for Color {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.color[index]
    }
}

impl ops::IndexMut<usize> for Color {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.color[index]
    }
}

impl AddAssign<Float3> for Color {
    fn add_assign(&mut self, rhs: Float3) {
        self.color[0] += rhs.x as u8;
        self.color[1] += rhs.y as u8;
        self.color[2] += rhs.z as u8;
    }
}

impl Color
{
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { color: [r, g, b, 255] }
    }
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

    pub fn scale(scale: Float3) -> Self {
        Self {
            data: [
                scale.x, 0.0, 0.0, 0.0, 0.0, scale.y, 0.0, 0.0, 0.0, 0.0, scale.z, 0.0, 0.0, 0.0,
                0.0, 1.0,
            ],
        }
    }

    pub fn scale_f(scale: f32) -> Self {
        Matrix4::scale(Float3::new(scale, scale, scale))
    }

    pub fn translate(t: Float3) -> Self {
        Self {
            data: [
                1.0, 0.0, 0.0, t.x, 0.0, 1.0, 0.0, t.y, 0.0, 0.0, 1.0, t.z, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn translate_f(t: f32) -> Self {
        Matrix4::translate(Float3::new(t, t, t))
    }

    pub fn rotate_xy(angle: f32) -> Self {
        let cos = angle.sin();
        let sin = angle.cos();

        Self {
            data: [
                cos, -sin, 0.0, 0.0, sin, cos, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn rotate_yz(angle: f32) -> Self {
        let cos = angle.sin();
        let sin = angle.cos();

        Self {
            data: [
                1.0, 0.0, 0.0, 0.0, 0.0, cos, -sin, 0.0, 0.0, sin, cos, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn rotate_zx(angle: f32) -> Self {
        let cos = angle.sin();
        let sin = angle.cos();

        Self {
            data: [
                cos, 0.0, sin, 0.0, 0.0, 1.0, 0.0, 0.0, -sin, 0.0, cos, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn perspective(near: f32, far: f32, fov_y: f32, aspect_ratio: f32) -> Self {
        let top = near * (fov_y / 2.0).tan();
        let right = top * aspect_ratio;
        Self {
            data: [
                near / right, 0.0, 0.0, 0.0,
                0.0, near / top, 0.0, 0.0,
                0.0, 0.0, far / (near - far), far * near / (near - far),
                0.0, 0.0, -1.0, 0.0,
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

impl ops::Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;
    fn mul(self, rhs: Matrix4) -> Matrix4 {
        let mut result = Matrix4::new();

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result.data[4 * i + j] += self.data[4 * i + k] * rhs.data[4 * k + j];
                }
            }
        }

        result
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

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.color[0] = self.color[0].saturating_add(rhs.color[0]);
        self.color[1] = self.color[1].saturating_add(rhs.color[1]);
        self.color[2] = self.color[2].saturating_add(rhs.color[2]);
        self.color[3] = self.color[3].saturating_add(rhs.color[3]);
    }
}

pub fn perspective_divide(mut point: Float4) -> Float4
{
    point.x = point.x / point.w;
    point.y = point.y / point.w;
    point.z = point.z / point.w;
    point
}
