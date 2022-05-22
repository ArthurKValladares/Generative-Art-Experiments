use math::vec::{Vec2, Vec3};

#[derive(Debug, Default)]
pub struct CompiledScene {
    pub positions: Vec<Vec3>,
    pub uvs: Vec<Vec2>,
    pub indices: Vec<u32>,
}
