pub mod id;
pub mod modified;

pub use id::ID;
pub use modified::Modified;
use ultraviolet::IVec3;

use crate::simulation::state::{
    physics::aabb::AABB,
    world::{cell::Cell, sector},
};

pub struct Sector {
    pub sector_id: sector::ID,
    pub modified: Modified,
    pub position: IVec3,
    pub aabb: AABB,
    pub cell_vec: Vec<Cell>,
}

impl Sector {
    pub fn get_cell_at(_coordinate: IVec3, sector: &Sector) -> &Cell {
        &sector.cell_vec[0]
    }
}
