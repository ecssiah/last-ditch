use crate::simulation::state::world::{block::Block, grid, object::ObjectManager, sector::Sector};
use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct SectorView {
    pub sector_index: usize,
    pub version: u64,
    pub world_position: Vec3,
    pub block_vec: Vec<Option<Block>>,
    pub object_manager: ObjectManager,
}

impl SectorView {
    pub fn new_from_sector(sector: &Sector) -> SectorView {
        SectorView {
            sector_index: sector.sector_index,
            version: sector.version,
            world_position: grid::grid_position_to_world_position(sector.grid_position),
            block_vec: sector.block_vec.clone(),
            object_manager: sector.object_manager.clone(),
        }
    }
}
