use crate::simulation::world::chunk::{self};

pub struct Graph {
    pub node_list: Vec<chunk::Node>,
    pub edge_list: Vec<chunk::Edge>,
    pub connection_list: Vec<chunk::Connection>,
}

impl Graph {
    pub fn new() -> Graph {
        let graph = Graph {
            node_list: Vec::new(),
            edge_list: Vec::new(),
            connection_list: Vec::new(),
        };

        graph
    }
}
