use crate::simulation::world::grid;
use glam::IVec3;

pub struct Connection {
    pub block_position: IVec3,
    pub direction: grid::Direction,
    pub group_id: usize,
}
