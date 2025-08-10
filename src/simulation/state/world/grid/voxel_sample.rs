use crate::simulation::state::world::{block, chunk, grid};
use glam::IVec3;

pub struct VoxelSample {
    pub t: f32,
    pub chunk_id: chunk::ID,
    pub block_id: block::ID,
    pub position: IVec3,
    pub enter_face: grid::Direction,
}
