pub mod geometry;
pub mod id;
pub mod modified;

pub use geometry::Geometry;
pub use id::ID;
pub use modified::Modified;

use crate::simulation::state::{
    physics::aabb::AABB,
    world::{block, chunk, grid},
};
use glam::IVec3;

pub struct Chunk {
    pub id: chunk::ID,
    pub modified: Modified,
    pub position: IVec3,
    pub aabb: AABB,
    pub geometry: chunk::Geometry,
    pub block_vec: Vec<block::Block>,
    pub visibility_vec: Vec<Vec<grid::Direction>>,
}

impl Chunk {
    pub fn get_block(&self, block_id: block::ID) -> Option<&block::Block> {
        self.block_vec.get(usize::from(block_id))
    }

    pub fn get_block_mut(&mut self, block_id: block::ID) -> Option<&mut block::Block> {
        self.block_vec.get_mut(usize::from(block_id))
    }
}
