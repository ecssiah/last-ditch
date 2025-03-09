use glam::{IVec3, Vec3};
use wgpu::Color;

#[derive(Debug, Copy, Clone)]
pub enum BlockType {
    Air,
    Translucent,
    Solid,
}

#[derive(Debug, Copy, Clone)]
pub struct Block {
    pub id: u32,
    pub chunk_id: u32,
    pub block_type: BlockType,
    pub local_position: IVec3,
    pub world_position: Vec3,
    pub color: Color,
}
