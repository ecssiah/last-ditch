use crate::simulation::state::{
    population::entity::{self, decision::plan},
    world::graph,
};
use glam::IVec3;

#[derive(Clone, Debug)]
pub struct LocalData {
    pub plan_id: plan::ID,
    pub entity_id: entity::ID,
    pub step_index: usize,
    pub start_position: IVec3,
    pub end_position: IVec3,
    pub level_0: graph::Level,
}

#[derive(Clone, Debug)]
pub struct RegionData {
    pub plan_id: plan::ID,
    pub entity_id: entity::ID,
    pub start_position: IVec3,
    pub end_position: IVec3,
    pub level_0: graph::Level,
    pub search_level: graph::Level,
}
