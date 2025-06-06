use crate::simulation::population::agent;
use glam::IVec3;

pub enum Result {
    Path {
        agent_id: agent::ID,
        path: Vec<IVec3>,
    },
}
