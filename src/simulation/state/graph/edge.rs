use crate::simulation::state::graph::Node;

pub enum Kind {
    Local,
    Regional,
}

pub struct Edge {
    pub node1: Node,
    pub node2: Node,
    pub level: u32,
    pub weight: f32,
    pub clearance: u32,
    pub kind: Kind,
}
