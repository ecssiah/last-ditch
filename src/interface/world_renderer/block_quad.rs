use ultraviolet::{Vec2, Vec3};

#[derive(Clone, Debug)]
pub struct BlockQuad {
    pub vertex_array: [Vec3; 4],
    pub uv_array: [Vec2; 4],
    pub normal: Vec3,
}