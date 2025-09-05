use crate::simulation::state::world::{block::Block, chunk};
use glam::Vec3;

#[derive(Clone, Debug)]
pub struct ChunkView {
    pub id: chunk::ID,
    pub world_position: Vec3,
    pub extent: Vec3,
    pub block_vec: Vec<Block>,
}

impl ChunkView {
    pub fn new() -> Self {
        Self {
            id: chunk::ID::MAX,
            world_position: Vec3::ZERO,
            extent: Vec3::ZERO,
            block_vec: Vec::new(),
        }
    }
}
