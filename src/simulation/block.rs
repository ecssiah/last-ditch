use cgmath::{Vector3, Vector4};

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
    pub position: Vector3<i32>,
    pub color: Vector4<f32>,
}
