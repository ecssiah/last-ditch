use crate::simulation::{observation::state_pair::StatePair, population::agent, time::Tick};
use glam::Vec3;

#[derive(Clone, Debug)]
pub struct AgentView {
    pub tick: StatePair<Tick>,
    pub id: agent::ID,
    pub kind: agent::Kind,
    pub height: f32,
    pub world_position: StatePair<Vec3>,
    pub target_world_position: StatePair<Vec3>,
}
