use crate::simulation::{observation::state_pair::StatePair, population::entity, time::Tick};
use glam::{Quat, Vec3};

#[derive(Clone, Debug)]
pub struct AgentView {
    pub id: entity::ID,

    pub tick: StatePair<Tick>,
    pub position: StatePair<Vec3>,
    pub orientation: StatePair<Quat>,
}