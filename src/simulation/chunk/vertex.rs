use glam::{Vec3, Vec4};

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub color: Vec4,
    pub light: f32,
}

impl Vertex {
}
