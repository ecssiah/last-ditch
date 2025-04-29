pub mod geometry;
pub mod id;

pub use geometry::Geometry;
pub use id::ID;

use crate::simulation::{
    consts::*,
    time::Tick,
    world::{block, chunk, grid},
    BLOCK_MAP,
};
use glam::IVec3;

pub struct Chunk {
    pub id: chunk::ID,
    pub tick: Tick,
    pub updated: bool,
    pub position: IVec3,
    pub geometry: chunk::Geometry,
    pub kind_list: Vec<block::Kind>,
    pub block_list: Box<[usize; CHUNK_VOLUME]>,
    pub light_list: Box<[block::Light; CHUNK_VOLUME]>,
    pub direction_list: Box<[grid::Direction; CHUNK_VOLUME]>,
    pub visibility_list: Box<[Vec<grid::Direction>; CHUNK_VOLUME]>,
    pub neighbor_list: Box<[Vec<grid::Direction>; CHUNK_VOLUME]>,
}

impl Chunk {
    pub fn get_block(&self, block_id: block::ID) -> Option<&block::Block> {
        if block::ID::is_valid(block_id) {
            let kind_id = self.block_list.get(usize::from(block_id))?;
            let kind = self.kind_list.get(usize::from(*kind_id))?;

            let block = BLOCK_MAP.get(&kind)?;

            Some(block)
        } else {
            None
        }
    }
}
