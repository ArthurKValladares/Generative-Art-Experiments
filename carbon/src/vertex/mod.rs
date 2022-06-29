use math::vec::{Vec2, Vec4};

// TODO: This will be defined in the shader later
#[repr(C)]
#[derive(Clone, Debug, Copy)]
pub struct Vertex {
    pub pos: Vec4,
    pub color: Vec4,
    pub uv: Vec2,
    pub pad: Vec2,
}
