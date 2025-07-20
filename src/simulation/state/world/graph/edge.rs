use crate::simulation::state::world::graph::Node;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Internal,
    External,
}

#[derive(Clone, Copy, Debug)]
pub struct Edge {
    pub node1: Node,
    pub node2: Node,
    pub kind: Kind,
    pub weight: u32,
    pub depth: usize,
}

impl Edge {
    pub fn new(node1: Node, node2: Node, kind: Kind, weight: u32, depth: usize) -> Self {
        Self {
            node1,
            node2,
            depth,
            weight,
            kind,
        }
    }

    pub fn flip(edge: &Edge) -> Edge {
        Edge::new(edge.node2, edge.node1, edge.kind, edge.weight, edge.depth)
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        let nodes_match = (self.node1 == other.node1 && self.node2 == other.node2)
            || (self.node1 == other.node2 && self.node2 == other.node1);

        nodes_match
            && self.kind == other.kind
            && self.weight == other.weight
            && self.depth == other.depth
    }
}

impl Eq for Edge {}
