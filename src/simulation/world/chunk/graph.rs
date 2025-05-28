use crate::simulation::world::chunk::Node;

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new() -> Graph {
        let graph = Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        };

        graph
    }
}
