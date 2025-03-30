use crate::simulation::{block, id::{block_id::BlockID, chunk_id::ChunkID, palette_id::PaletteID}, time::Tick, BLOCKS, CHUNK_VOLUME};
use glam::IVec3;

pub mod mesh;
pub mod vertex;

pub struct Chunk {
    pub last_update: Tick,
    pub id: ChunkID,
    pub position: IVec3,
    pub palette: Vec<block::Kind>,
    pub palette_ids: Vec<PaletteID>,
    pub meta: Box<[block::Meta; CHUNK_VOLUME]>,
    pub light: Box<[block::LightLevel; CHUNK_VOLUME]>,
    pub mesh: mesh::Mesh,
}

impl Chunk {
    pub fn get_block(&self, block_id: BlockID) -> Option<&block::Block> {
        let palette_id = self.palette_ids.get(usize::from(block_id))?;
        let kind = self.palette.get(usize::from(*palette_id))?;

        let block = BLOCKS.get(&kind)?;

        Some(block)
    }

    pub fn get_meta(&self, block_id: BlockID) -> Option<&block::Meta> {
        let meta = self.meta.get(usize::from(block_id))?;

        Some(meta)
    }

    pub fn get_meta_mut(&mut self, block_id: BlockID) -> Option<&mut block::Meta> {
        let meta = self.meta.get_mut(usize::from(block_id))?;

        Some(meta)
    }
}
