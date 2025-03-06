use cgmath::{Vector3, Vector4};

#[derive(Debug, Copy, Clone)]
pub enum BlockType {
    Empty,
    Solid,
}

#[derive(Debug, Copy, Clone)]
pub struct Block {
    pub id: u64,
    pub chunk_id: u64,
    pub block_type: BlockType,
    pub position: Vector3<i64>,
    pub color: Vector4<f32>,
}
