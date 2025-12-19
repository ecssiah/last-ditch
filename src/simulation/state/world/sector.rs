use crate::simulation::state::world::Cell;
use ultraviolet::IVec3;

pub struct Sector {
    pub sector_id: usize,
    pub version: u64,
    pub grid_position: IVec3,
    pub cell_vec: Vec<Cell>,
}
