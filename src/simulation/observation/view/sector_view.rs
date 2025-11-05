use crate::simulation::state::world::{block::Block, sector};
use glam::Vec3;

#[derive(Clone, Debug)]
pub struct SectorView {
    pub id: sector::ID,
    pub world_position: Vec3,
    pub extent: Vec3,
    pub block_vec: Vec<Block>,
}

impl SectorView {
    pub fn new() -> Self {
        Self {
            id: sector::ID::MAX,
            world_position: Vec3::ZERO,
            extent: Vec3::ZERO,
            block_vec: Vec::new(),
        }
    }
}
