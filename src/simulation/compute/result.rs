use glam::IVec3;
use crate::simulation::population::agent;

pub enum Result {
    Path {
        agent_id: agent::ID,
        path: Vec<IVec3>,
    },
}
