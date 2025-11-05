pub mod id;
pub mod modified;

pub use id::ID;
pub use modified::Modified;

use crate::simulation::state::{
    physics::aabb::AABB,
    world::{block, sector},
};
use glam::IVec3;

pub struct Sector {
    pub id: sector::ID,
    pub modified: Modified,
    pub position: IVec3,
    pub aabb: AABB,
    pub block_vec: Vec<block::Block>,
}
