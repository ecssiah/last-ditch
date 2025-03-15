use crate::simulation::{block, CHUNK_VOLUME};
use glam::IVec3;

#[derive(Debug)]
pub struct Chunk {
    pub last_update: u64,
    pub position: IVec3,
    pub palette: Vec<block::Kind>,
    pub palette_ids: Box<[u32; CHUNK_VOLUME as usize]>,
    pub meta: Box<[block::Meta; CHUNK_VOLUME as usize]>,
}