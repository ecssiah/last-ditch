use crate::simulation::state::world::{cell, grid, sector};
use glam::{IVec3, Vec3};

#[derive(Debug)]
pub struct CellSample {
    pub t: f32,
    pub position: IVec3,
    pub world_position: Vec3,
    pub sector_id: sector::ID,
    pub cell_id: cell::ID,
    pub enter_face_direction: grid::Direction,
}
