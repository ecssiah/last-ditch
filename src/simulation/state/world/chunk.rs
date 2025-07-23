pub mod id;
pub mod modified;

pub use id::ID;
pub use modified::Modified;

use crate::simulation::state::{
    physics::aabb::AABB,
    world::{block, chunk},
};
use glam::IVec3;

pub struct Chunk {
    pub id: chunk::ID,
    pub modified: Modified,
    pub position: IVec3,
    pub aabb: AABB,
    pub block_vec: Vec<block::Block>,
}
