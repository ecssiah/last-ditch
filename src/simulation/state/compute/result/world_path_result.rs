use crate::simulation::state::population::agent;
use glam::IVec3;

#[derive(Debug)]
pub struct WorldPathResult {
    pub agent_id: agent::ID,
    pub path: Vec<IVec3>,
}
