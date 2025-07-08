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

impl Edge {
    pub fn new(node1: Node, node2: Node, depth: usize, weight: u32, kind: Kind) -> Self {
        Self {
            node1,
            node2,
            depth,
            weight,
            kind,
        }
    }
}
