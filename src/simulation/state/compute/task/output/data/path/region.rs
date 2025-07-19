use crate::simulation::state::{
    population::entity::{self, decision::plan},
    world::graph::Edge,
};

#[derive(Clone, Debug)]
pub struct Region {
    pub plan_id: plan::ID,
    pub entity_id: entity::ID,
    pub edge_vec: Vec<Edge>,
}
