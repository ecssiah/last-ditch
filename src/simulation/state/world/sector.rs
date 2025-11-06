pub mod id;
pub mod modified;

pub use id::ID;
pub use modified::Modified;

use crate::simulation::state::{
    physics::aabb::AABB,
    world::{cell::Cell, sector},
};
use glam::IVec3;

pub struct Sector {
    pub sector_id: sector::ID,
    pub modified: Modified,
    pub position: IVec3,
    pub aabb: AABB,
    pub cell_vec: Vec<Cell>,
}
