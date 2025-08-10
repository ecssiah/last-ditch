use crate::simulation::state::world::{block, chunk, grid};
use glam::{IVec3, Vec3};

pub struct BlockSample {
    pub t: f32,
    pub position: IVec3,
    pub world_position: Vec3,
    pub chunk_id: chunk::ID,
    pub block_id: block::ID,
    pub enter_face_direction: grid::Direction,
}
