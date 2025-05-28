use crate::simulation::population::agent;
use glam::IVec3;

pub enum Task {
    Path {
        agent_id: agent::ID,
        from: IVec3,
        to: IVec3,
    },
}
