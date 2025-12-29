use crate::simulation::state::world::{
    grid,
    object::{Block, Door, Ladder, Stairs},
    sector::Sector,
};
use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct SectorView {
    pub sector_index: usize,
    pub version: u64,
    pub world_position: Vec3,
    pub block_vec: Vec<Option<Block>>,
    pub door_vec: Vec<Option<Door>>,
    pub ladder_vec: Vec<Option<Ladder>>,
    pub stairs_vec: Vec<Option<Stairs>>,
}

impl SectorView {
    pub fn new_from_sector(sector: &Sector) -> SectorView {
        SectorView {
            sector_index: sector.sector_index,
            version: sector.version,
            world_position: grid::grid_position_to_world_position(sector.grid_position),
            block_vec: sector.block_vec.clone(),
            door_vec: sector.door_vec.clone(),
            ladder_vec: sector.ladder_vec.clone(),
            stairs_vec: sector.stairs_vec.clone(),
        }
    }
}
