use crate::simulation::{observation::state_pair::StatePair, state::population::agent};
use glam::Vec3;

#[derive(Clone, Debug)]
pub struct AgentView {
    pub id: agent::ID,
    pub kind: agent::Kind,
    pub height: f32,
    pub world_position: StatePair<Vec3>,
    pub target_world_position: StatePair<Vec3>,
}
