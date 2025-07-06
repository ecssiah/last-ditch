use crate::simulation::state::{population::entity, world::chunk};
use glam::IVec3;

#[derive(Debug)]
pub struct BlockPathTask {
    pub agent_id: entity::ID,
    pub chunk_id: chunk::ID,
    pub start: IVec3,
    pub end: IVec3,
}
