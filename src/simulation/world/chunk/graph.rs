use glam::IVec3;

use crate::simulation::world::chunk::{self};

pub struct Graph {
    pub node_list: Vec<chunk::Node>,
    pub connection_list: Vec<chunk::Connection>,
}

impl Graph {
    pub fn new() -> Graph {
        let graph = Graph {
            node_list: Vec::new(),
            connection_list: Vec::new(),
        };

        graph
    }

    pub fn get_node_index(&self, grid_position: IVec3) -> Option<usize> {
        self.node_list
            .iter()
            .position(|node| node.grid_position == grid_position)
    }
}
