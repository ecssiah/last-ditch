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
}

impl Default for Level {
    fn default() -> Self {
        Self::new()
    }
}
