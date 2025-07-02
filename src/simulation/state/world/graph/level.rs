use crate::simulation::state::world::graph::{Edge, Node};
use glam::IVec3;
use std::collections::HashMap;

pub struct Level {
    pub node_vec: HashMap<IVec3, Node>,
    pub edge_vec: HashMap<(IVec3, IVec3), Edge>,
}

impl Level {
    pub fn new() -> Self {
        Self {
            node_vec: HashMap::new(),
            edge_vec: HashMap::new(),
        }
    }
}

impl Default for Level {
    fn default() -> Self {
        Self::new()
    }
}
