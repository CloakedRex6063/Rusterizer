use crate::math::{Float3};

pub struct PointLight
{
    pub pos: Float3,
    pub intensity: f32,
    pub color: Float3,
    pub range: f32,
}

pub struct DirectionalLight
{
    pub direction: Float3,
    pub intensity: f32,
    pub color: Float3,
    pub cast_shadow: bool,
}