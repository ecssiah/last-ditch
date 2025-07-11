use crate::simulation::state::population::entity::{self, decision::plan};
use glam::IVec3;

#[derive(Clone, Debug)]
pub struct Region {
    pub plan_id: plan::ID,
    pub agent_id: entity::ID,
    pub position_vec: Vec<IVec3>,
}
