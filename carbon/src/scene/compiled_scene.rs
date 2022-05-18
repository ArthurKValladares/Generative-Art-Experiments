use math::vec::Vec3;

#[derive(Debug, Default)]
pub struct CompiledScene {
    pub positions: Vec<Vec3>,
    pub indices: Vec<u32>,
}
