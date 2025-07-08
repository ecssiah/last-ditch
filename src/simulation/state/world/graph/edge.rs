use crate::simulation::state::world::graph::Node;

#[derive(Clone, Copy, Debug)]
pub enum Kind {
    Internal,
    External,
}

#[derive(Clone, Debug)]
pub struct Edge {
    pub node1: Node,
    pub node2: Node,
    pub depth: usize,
    pub weight: u32,
    pub kind: Kind,
}
