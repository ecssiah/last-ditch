use crate::simulation::state::world::graph::{Edge, Node};
use glam::IVec3;
use std::collections::HashMap;

pub struct Level {
    pub node_map: HashMap<IVec3, Node>,
    pub edge_map: HashMap<(IVec3, IVec3), Edge>,
}

impl Level {
    pub fn new() -> Self {
        Self {
            node_map: HashMap::new(),
            edge_map: HashMap::new(),
        }
    }

    pub fn neighbors(&self, position: IVec3) -> Vec<Node> {
        let mut node_vec = Vec::new();

        for ((position1, position2), edge) in &self.edge_map {
            if &position == position1 {
                node_vec.push(edge.node2);
            } else if &position == position2 {
                node_vec.push(edge.node1);
            }
        }

        node_vec
    }
}

impl Default for Level {
    fn default() -> Self {
        Self::new()
    }
}
