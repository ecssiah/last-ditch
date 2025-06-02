use crate::simulation::world;
use glam::IVec3;
use std::collections::HashMap;

pub struct Graph {
    pub node_map: HashMap<IVec3, world::Node>,
}

impl Graph {
    pub fn new() -> Self {
        let graph = Graph {
            node_map: HashMap::new(),
        };

        graph
    }

    pub fn has_node(&self, grid_position: IVec3) -> bool {
        self.node_map.contains_key(&grid_position)
    }

    pub fn get_node(&self, grid_position: IVec3) -> Option<&world::Node> {
        self.node_map.get(&grid_position)
    }
}
