use crate::simulation::state::{
    population::entity::{self, decision::plan},
    world::graph::Edge,
};

#[derive(Clone, Debug)]
pub struct LocalData {
    pub plan_id: plan::ID,
    pub entity_id: entity::ID,
    pub step_index: usize,
    pub edge_vec: Vec<Edge>,
}

#[derive(Clone, Debug)]
pub struct RegionData {
    pub plan_id: plan::ID,
    pub entity_id: entity::ID,
    pub edge_vec: Vec<Edge>,
}
