use crate::simulation::state::world::graph::Node;

#[derive(Clone, Copy, Debug)]
pub enum Kind {
    Local,
    Regional,
}

#[derive(Clone, Debug)]
pub struct Edge {
    pub node1: Node,
    pub node2: Node,
    pub level: u32,
    pub weight: u32,
    pub clearance: u32,
    pub kind: Kind,
}
