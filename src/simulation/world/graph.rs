use crate::simulation::world::chunk;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct Graph {
    pub node_map: HashMap<chunk::ID, chunk::Node>,
    pub edge_map: HashMap<chunk::EdgeKey, chunk::Edge>,
    pub node_edge_map: HashMap<chunk::ID, HashSet<chunk::EdgeKey>>,
    pub chunk_graph_map: HashMap<chunk::ID, chunk::Graph>,
}

impl Graph {
    pub fn new() -> Self {
        let graph = Graph {
            node_map: HashMap::new(),
            edge_map: HashMap::new(),
            node_edge_map: HashMap::new(),
            chunk_graph_map: HashMap::new(),
        };

        graph
    }

    pub fn add_chunk_node(
        &mut self,
        chunk_id: chunk::ID,
        chunk_node: chunk::Node,
    ) -> Option<chunk::Node> {
        self.node_map.insert(chunk_id, chunk_node)
    }

    pub fn add_chunk_graph(
        &mut self,
        chunk_id: chunk::ID,
        chunk_graph: chunk::Graph,
    ) -> Option<chunk::Graph> {
        self.chunk_graph_map.insert(chunk_id, chunk_graph)
    }

    pub fn get_chunk_graph(&self, chunk_id: chunk::ID) -> Option<&chunk::Graph> {
        self.chunk_graph_map.get(&chunk_id)
    }
}
