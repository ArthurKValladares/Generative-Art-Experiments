use crate::camera::Camera;
use math::vec::{Vec2, Vec3, Vec4};

#[derive(Debug, Default)]
pub struct CompiledScene {
    pub positions: Vec<Vec4>,
    pub colors: Vec<Vec4>,
    pub uvs: Vec<Vec2>,
    pub normals: Vec<Vec3>,
    pub indices: Vec<u32>,
    pub material_indices: Vec<u32>,
    pub cameras: Vec<Camera>,
}
