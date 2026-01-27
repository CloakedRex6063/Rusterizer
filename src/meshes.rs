use std::path::Path;
use gltf::Document;
use crate::image_view::Texture;
use crate::math::{Color, Float2, Float3};

pub struct Model
{
    pub meshes: Vec<Mesh>,
    pub textures: Vec<Texture>
}

impl Model
{
    pub fn from_file(path: &Path) -> Model
    {
        let (document, buffers, images) = gltf::import(path).unwrap();

        let meshes = Self::load_meshes(&document, &*buffers);
        let textures = Self::load_textures(&document, &*images);

        Model{
            meshes,
            textures,
        }
    }

    fn load_meshes(document: &Document, buffers: &[gltf::buffer::Data]) -> Vec<Mesh>
    {
        let mut meshes: Vec<Mesh> = vec![];
        for mesh in document.meshes()
        {
            for primitive in mesh.primitives() {
                let mut mesh = Mesh{
                    positions: vec![],
                    indices: vec![],
                    uvs: vec![],
                    normals: vec![],
                    albedo_texture_index: None,
                    normal_texture_index: None,
                    metal_rough_texture_index: None,
                    occlusion_texture_index: None,
                    emissive_texture_index: None,
                };

                let material = primitive.material();
                let pbr = material.pbr_metallic_roughness();

                mesh.albedo_texture_index = pbr
                    .base_color_texture()
                    .map(|info| info.texture().index());

                mesh.metal_rough_texture_index = pbr
                    .metallic_roughness_texture()
                    .map(|info| info.texture().index());

                mesh.normal_texture_index = material
                    .normal_texture()
                    .map(|info| info.texture().index());

                mesh.occlusion_texture_index = material
                    .occlusion_texture()
                    .map(|info| info.texture().index());

                mesh.emissive_texture_index = material
                    .emissive_texture()
                    .map(|info| info.texture().index());

                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                if let Some(indices_reader) = reader.read_indices() {
                    indices_reader.into_u32().for_each(|i| mesh.indices.push(i));
                }
                if let Some(positions_reader) = reader.read_positions() {
                    positions_reader.for_each(|p| mesh.positions.push(Float3::new(p[0], p[1], p[2])));
                }
                if let Some(normals_reader) = reader.read_normals() {
                    normals_reader.for_each(|n| mesh.normals.push(Float3::new(n[0], n[1], n[2])));
                }
                if let Some(tex_coord_reader) = reader.read_tex_coords(0) {
                    tex_coord_reader
                        .into_f32()
                        .for_each(|tc| mesh.uvs.push(Float2::new(tc[0], tc[1])));
                }
                meshes.push(mesh);
            }
        }

        meshes
    }

    fn load_textures(document: &Document, images: &[gltf::image::Data]) -> Vec<Texture> {
        let mut textures: Vec<Texture> = vec![];

        for texture in document.textures() {
            let image = &images[texture.source().index()];

            let pixels: Vec<Color> = match image.format {
                gltf::image::Format::R8G8B8A8 => {
                    image.pixels
                        .chunks_exact(4)
                        .map(|c| Color::new(c[0], c[1], c[2], c[3]))
                        .collect()
                }
                gltf::image::Format::R8G8B8 => {
                    image.pixels
                        .chunks_exact(3)
                        .map(|c| Color::new(c[0], c[1], c[2], 255))
                        .collect()
                }
                gltf::image::Format::R8G8 => {
                    image.pixels
                        .chunks_exact(2)
                        .map(|c| Color::new(c[0], c[1], 0, 255))
                        .collect()
                }
                gltf::image::Format::R8 => {
                    image.pixels
                        .iter()
                        .map(|&r| Color::new(r, r, r, 255))
                        .collect()
                }
                _ => {
                    eprintln!("Unsupported texture format: {:?}", image.format);
                    vec![Color::default(); (image.width * image.height) as usize]
                }
            };

            textures.push(Texture {
                width: image.width,
                height: image.height,
                pixels,
            });
        }

        textures
    }
}

pub struct Mesh {
    pub positions: Vec<Float3>,
    pub indices: Vec<u32>,
    pub uvs: Vec<Float2>,
    pub normals: Vec<Float3>,
    pub albedo_texture_index: Option<usize>,
    pub normal_texture_index: Option<usize>,
    pub metal_rough_texture_index: Option<usize>,
    pub occlusion_texture_index: Option<usize>,
    pub emissive_texture_index: Option<usize>,
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

        let indices: Vec<u32> = vec![
            // -X face
            0, 2, 1, 1, 2, 3, // +X face
            4, 5, 6, 6, 5, 7, // -Y face
            8, 9, 10, 10, 9, 11, // +Y face
            12, 14, 13, 14, 15, 13, // -Z face
            16, 18, 17, 17, 18, 19, // +Z face
            20, 21, 22, 21, 23, 22,
        ];

        let uvs = vec![
            // -X face
            Float2::new(0.0, 0.0),
            Float2::new(1.0, 0.0),
            Float2::new(0.0, 1.0),
            Float2::new(1.0, 1.0),
            // +X face
            Float2::new(0.0, 0.0),
            Float2::new(1.0, 0.0),
            Float2::new(0.0, 1.0),
            Float2::new(1.0, 1.0),
            // -Y face
            Float2::new(0.0, 0.0),
            Float2::new(1.0, 0.0),
            Float2::new(0.0, 1.0),
            Float2::new(1.0, 1.0),
            // +Y face
            Float2::new(0.0, 0.0),
            Float2::new(1.0, 0.0),
            Float2::new(0.0, 1.0),
            Float2::new(1.0, 1.0),
            // -Z face
            Float2::new(0.0, 0.0),
            Float2::new(1.0, 0.0),
            Float2::new(0.0, 1.0),
            Float2::new(1.0, 1.0),
            // +Z face
            Float2::new(0.0, 0.0),
            Float2::new(1.0, 0.0),
            Float2::new(0.0, 1.0),
            Float2::new(1.0, 1.0),
        ];

        let mesh = Mesh {
            positions,
            indices,
            uvs,
            normals: vec![],
            albedo_texture_index: None,
            normal_texture_index: None,
            metal_rough_texture_index: None,
            occlusion_texture_index: None,
            emissive_texture_index: None,
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

        let indices = vec![
            0, 1, 2, 2, 1, 3,
        ];

        let uvs = vec![
            Float2::new(0.0, 0.0),
            Float2::new(1.0, 0.0),
            Float2::new(0.0, 1.0),
            Float2::new(1.0, 1.0),
        ];

        let mesh = Mesh{
            positions,
            indices,
            uvs,
            normals: vec![],
            albedo_texture_index: None,
            normal_texture_index: None,
            metal_rough_texture_index: None,
            occlusion_texture_index: None,
            emissive_texture_index: None,
        };

        Self{
            mesh,
        }
    }
}