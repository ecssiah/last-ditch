use crate::simulation::state::population::entity;
use glam::IVec3;

#[derive(Clone, Copy, Debug)]
pub struct Regional {
    pub agent_id: entity::ID,
    pub start_position: IVec3,
    pub end_position: IVec3,
}
