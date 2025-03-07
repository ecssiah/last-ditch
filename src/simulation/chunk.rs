use super::block::Block;
use crate::consts::CHUNK_VOLUME;
use cgmath::Vector3;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub id: u32,
    pub position: Vector3<i32>,
    pub modified: bool,
    pub blocks: Box<[Block; CHUNK_VOLUME as usize]>,
}
