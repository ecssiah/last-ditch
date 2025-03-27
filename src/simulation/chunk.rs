use crate::simulation::{block, world, BLOCKS, CHUNK_VOLUME};
use glam::{IVec3, Vec3, Vec4};

pub type ChunkID = usize;
pub type PaletteID = usize;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub color: Vec4,
    pub ao: f32,
}

#[derive(Debug, Default, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

pub struct Chunk {
    pub last_update: world::Tick,
    pub id: ChunkID,
    pub position: IVec3,
    pub palette: Vec<block::Kind>,
    pub palette_ids: Box<[PaletteID; CHUNK_VOLUME]>,
    pub meta: Box<[block::Meta; CHUNK_VOLUME]>,
    pub light: Box<[block::LightLevel; CHUNK_VOLUME]>,
    pub mesh: Mesh,
}

impl Chunk {
    pub fn get_block(&self, block_id: block::BlockID) -> Option<&block::Block> {
        let palette_id = self.palette_ids[block_id];
        let kind = self.palette[palette_id];

        let block = BLOCKS.get(&kind)?;

        Some(block)
    }
}
