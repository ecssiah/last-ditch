use crate::simulation::world::{chunk, grid};
use glam::IVec3;

pub struct Connection {
    pub chunk_id: chunk::ID,
    pub direction: grid::Direction,
    pub clearance: u32,
    pub group_id: u32,
    pub source: IVec3,
    pub target: IVec3,
}
