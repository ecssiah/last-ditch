use crate::simulation::state::{
    population::entity::{self, decision::plan},
    world::graph::Edge,
};

#[derive(Clone, Debug)]
pub struct Local {
    pub plan_id: plan::ID,
    pub entity_id: entity::ID,
    pub path_index: usize,
    pub edge_vec: Vec<Edge>,
}
