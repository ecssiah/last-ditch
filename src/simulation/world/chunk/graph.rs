use crate::simulation::world::chunk::{self};

pub struct Graph {
    pub nodes: Vec<chunk::Node>,
    pub edges: Vec<Vec<usize>>,
    pub connections: Vec<chunk::Connection>,
}

impl Graph {
    pub fn new() -> Graph {
        let graph = Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            connections: Vec::new(),
        };

        graph
    }
}
