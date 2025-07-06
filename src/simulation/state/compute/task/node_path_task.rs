use crate::simulation::state::{population::entity, world::graph::Node};

#[derive(Debug)]
pub struct NodePathTask {
    pub agent_id: entity::ID,
    pub node_vec: Vec<Node>,
}
