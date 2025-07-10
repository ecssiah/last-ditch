use crate::simulation::state::population::entity;
use glam::IVec3;

#[derive(Clone, Debug)]
pub struct Regional {
    pub agent_id: entity::ID,
    pub position_vec: Vec<IVec3>,
}
