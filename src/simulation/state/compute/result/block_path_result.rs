use crate::simulation::state::population::entity;
use glam::IVec3;

#[derive(Debug)]
pub struct BlockPathResult {
    pub agent_id: entity::ID,
    pub position_vec: Vec<IVec3>,
}
