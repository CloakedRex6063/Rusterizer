use crate::math::{Float3, Float4};

pub struct Mesh {
    pub positions: Vec<Float3>,
    pub indices: Vec<u32>,
    pub colors: Vec<Float4>,
}

pub struct Cube
{
    pub mesh: Mesh
}

impl Cube {
    pub fn new() -> Self
    {
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

        Self{
            mesh,
        }
    }
}

pub struct Square {
    pub mesh: Mesh
}

impl Square {
    pub fn new() -> Self
    {
        let positions = vec![
            Float3::new(-1.0, -1.0, 0.0),
            Float3::new(1.0, -1.0, 0.0),
            Float3::new(-1.0, 1.0, 0.0),
            Float3::new(1.0, 1.0, 0.0),
        ];

        let colors = vec![
            Float4::new(0.0, 0.0, 0.0, 1.0),
            Float4::new(1.0, 0.0, 0.0, 1.0),
            Float4::new(0.0, 1.0, 0.0, 1.0),
            Float4::new(0.0, 0.0, 1.0, 1.0),
        ];

        let indices = vec![
            0, 1, 2, 2, 1, 3,
        ];

        let mesh = Mesh{
            positions,
            indices,
            colors,
        };

        Self{
            mesh,
        }
    }
}