use rapier3d::na::Point3;
use wgpu::Color;

#[derive(Debug, Copy, Clone)]
pub enum BlockType {
    None,
    Translucent,
    Solid,
}

#[derive(Debug, Copy, Clone)]
pub struct Block {
    pub id: u32,
    pub chunk_id: u32,
    pub block_type: BlockType,
    pub local_position: Point3<i32>,
    pub world_position: Point3<f32>,
    pub color: Color,
}
