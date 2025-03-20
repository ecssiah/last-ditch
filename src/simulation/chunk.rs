use crate::simulation::{block, CHUNK_VOLUME};
use glam::IVec3;

pub type ChunkID = usize;
pub type PaletteID = usize;

#[derive(Debug)]
pub struct Chunk {
    pub last_update: u64,
    pub id: ChunkID,
    pub position: IVec3,
    pub palette: Vec<block::Kind>,
    pub palette_ids: Box<[PaletteID; CHUNK_VOLUME]>,
    pub meta: Box<[block::Meta; CHUNK_VOLUME]>,
    pub light: Box<[block::LightLevel; CHUNK_VOLUME]>,
}
