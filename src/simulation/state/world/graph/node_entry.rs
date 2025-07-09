use crate::simulation::state::world::graph::Node;

#[derive(Eq, PartialEq)]
pub struct NodeEntry {
    pub cost: u32,
    pub node: Node,
}

impl NodeEntry {
    pub fn new(cost: u32, node: Node) -> Self {
        NodeEntry { cost, node }
    }
}

impl Ord for NodeEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost).then_with(|| {
            self.node
                .position
                .to_array()
                .cmp(&other.node.position.to_array())
        })
    }
}

impl PartialOrd for NodeEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
