use crate::simulation::population::agent;
use glam::IVec3;

pub struct WorldPathResult {
    pub agent_id: agent::ID,
    pub path: Vec<IVec3>,
}
