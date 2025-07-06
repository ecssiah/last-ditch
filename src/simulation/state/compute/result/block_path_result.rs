use crate::simulation::state::{population::entity, world::chunk};
use glam::IVec3;

#[derive(Debug)]
pub struct BlockPathResult {
    pub agent_id: entity::ID,
    pub chunk_id: chunk::ID,
    pub position_vec: Vec<IVec3>,
}
