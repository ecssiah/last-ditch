use crate::simulation::state::population::entity;
use glam::IVec3;

#[derive(Debug)]
pub struct WorldPathResult {
    pub agent_id: entity::ID,
    pub path: Vec<IVec3>,
}
