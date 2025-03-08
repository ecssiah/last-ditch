use super::{block::Block, CHUNK_VOLUME};
use glam::{IVec3, Vec3};

#[derive(Debug, Clone)]
pub struct Chunk {
    pub id: u32,
    pub local_position: IVec3,
    pub world_position: Vec3,
    pub modified: bool,
    pub blocks: Box<[Block; CHUNK_VOLUME as usize]>,
}
