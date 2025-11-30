use crate::simulation::state::world::grid;
use ultraviolet::{IVec3, Vec3};

#[derive(Debug)]
pub struct CellSample {
    pub t: f32,
    pub grid_position: IVec3,
    pub world_position: Vec3,
    pub sector_id: usize,
    pub cell_id: usize,
    pub direction_entered: grid::Direction,
}
