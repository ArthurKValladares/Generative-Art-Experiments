use crate::camera::Camera;
use math::{
    mat::Mat4,
    vec::{Vec2, Vec3, Vec4},
};

#[derive(Debug)]
pub struct PbrMetallicRoughness {
    pub base_color_factor: Vec4,
    pub texture_index: Option<usize>,
}

#[derive(Debug)]
pub struct Material {
    metallic_roughness: PbrMetallicRoughness,
}

impl Material {
    pub fn new(mat: &gltf::material::Material) -> Self {
        let metallic_roughness = {
            let gltf_metallic_roughness = mat.pbr_metallic_roughness();

            let base_color_factor = gltf_metallic_roughness.base_color_factor().into();
            let texture_index = gltf_metallic_roughness
                .base_color_texture()
                .map(|info| info.texture().index());
            PbrMetallicRoughness {
                base_color_factor,
                texture_index,
            }
        };
        Self { metallic_roughness }
    }
}

#[derive(Debug, Default)]
pub struct MeshDraw {
    pub start_idx: u32,
    pub num_indices: u32,
    pub material_idx: u32,
    // TODO: This is inneficient here, duplication it too much
    pub transform_matrix: Mat4,
}

#[derive(Debug, Default)]
pub struct CompiledScene {
    pub positions: Vec<Vec4>,
    pub colors: Vec<Vec4>,
    pub uvs: Vec<Vec2>,
    pub normals: Vec<Vec3>,
    pub indices: Vec<u32>,
    pub materials: Vec<Material>,
    pub mesh_draws: Vec<MeshDraw>,
    pub cameras: Vec<Camera>,
}
