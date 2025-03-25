use crate::simulation::{block, BLOCKS, CHUNK_VOLUME};
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

impl Chunk {
    pub fn get_block(&self, block_id: block::BlockID) -> Option<&block::Block> {
        let palette_id = self.palette_ids[block_id];
        let kind = self.palette[palette_id];

        let block = BLOCKS.get(&kind)?;

        Some(block)
    }
}
