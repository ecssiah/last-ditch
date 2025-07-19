use crate::simulation::state::world::graph::Node;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Internal,
    External,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Edge {
    pub node1: Node,
    pub node2: Node,
    pub kind: Kind,
    pub weight: u32,
    pub depth: usize,
}

impl Edge {
    pub fn new(node1: Node, node2: Node, kind: Kind, weight: u32, depth: usize) -> Self {
        let (node1, node2) = if node1.position.to_array() <= node2.position.to_array() {
            (node1, node2)
        } else {
            (node2, node1)
        };

        Self {
            node1,
            node2,
            depth,
            weight,
            kind,
        }
    }
}
