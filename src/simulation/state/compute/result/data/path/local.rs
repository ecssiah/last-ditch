use crate::simulation::state::{
    population::entity::{self, decision::plan},
    world::chunk,
};
use glam::IVec3;

#[derive(Clone, Debug)]
pub struct Local {
    pub plan_id: plan::ID,
    pub entity_id: entity::ID,
    pub chunk_id: chunk::ID,
    pub position_vec: Vec<IVec3>,
}
