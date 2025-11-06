use crate::simulation::state::world::{cell::Cell, sector};
use glam::Vec3;

#[derive(Clone, Debug)]
pub struct SectorView {
    pub sector_id: sector::ID,
    pub world_position: Vec3,
    pub extent: Vec3,
    pub cell_vec: Vec<Cell>,
}

impl SectorView {
    pub fn new() -> Self {
        Self {
            sector_id: sector::ID::MAX,
            world_position: Vec3::ZERO,
            extent: Vec3::ZERO,
            cell_vec: Vec::new(),
        }
    }
}
