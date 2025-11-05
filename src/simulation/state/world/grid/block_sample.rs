use crate::simulation::state::world::{block, grid, sector};
use glam::{IVec3, Vec3};

#[derive(Debug)]
pub struct BlockSample {
    pub t: f32,
    pub position: IVec3,
    pub world_position: Vec3,
    pub sector_id: sector::ID,
    pub block_id: block::ID,
    pub enter_face_direction: grid::Direction,
}
