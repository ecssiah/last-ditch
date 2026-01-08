use crate::simulation::state::world::{block::Block, grid, sector::{Sector, sector_index::SectorIndex}};
use ultraviolet::Vec3;

#[derive(Clone)]
pub struct SectorView {
    pub sector_index: SectorIndex,
    pub version: u64,
    pub world_position: Vec3,
    pub block_vec: Vec<Option<Block>>,
}

impl SectorView {
    pub fn new_from_sector(sector: &Sector) -> Self {
        Self {
            sector_index: sector.sector_index,
            version: sector.version,
            world_position: grid::grid_position_to_world_position(sector.grid_position),
            block_vec: sector.block_vec.clone(),
        }
    }
}
