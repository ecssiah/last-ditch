use crate::simulation::population::agent;
use glam::IVec3;

#[derive(Debug)]
pub struct ChunkPathResult {
    pub agent_id: agent::ID,
    pub path: Vec<IVec3>,
}
