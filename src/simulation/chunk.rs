use cgmath::Vector3;

use super::block::Block;
use crate::consts::CHUNK_VOLUME;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub id: u64,
    pub position: Vector3<i64>,
    pub modified: bool,
    pub blocks: Box<[Block; CHUNK_VOLUME as usize]>,
}
