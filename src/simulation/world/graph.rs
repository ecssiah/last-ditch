use glam::IVec3;

use crate::simulation::world::{block, chunk};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct Graph {
    pub node_map: HashMap<chunk::ID, chunk::Node>,
    pub edge_map: HashMap<chunk::edge::Key, chunk::Edge>,
    pub node_edge_map: HashMap<chunk::ID, HashSet<chunk::edge::Key>>,
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

    pub fn get_node_chunk_ids(&self) -> impl Iterator<Item = chunk::ID> + '_ {
        self.node_map.keys().copied()
    }

    pub fn get_chunk_node(&self, chunk_id: chunk::ID) -> Option<&chunk::Node> {
        self.node_map.get(&chunk_id)
    }

    pub fn get_chunk_node_mut(&mut self, chunk_id: chunk::ID) -> Option<&mut chunk::Node> {
        self.node_map.get_mut(&chunk_id)
    }

    pub fn clear_edges(&mut self, chunk_id: chunk::ID) {
        if let Some(keys) = self.node_edge_map.remove(&chunk_id) {
            for key in keys {
                self.edge_map.remove(&key);
            }
        }
    }

    pub fn add_edge(
        &mut self,
        chunk_id1: chunk::ID,
        block_id1: block::ID,
        block_position1: IVec3,
        chunk_id2: chunk::ID,
        block_id2: block::ID,
        block_position2: IVec3,
        clearance: u32,
        cost: f32,
    ) {
        let key = chunk::edge::Key::new(chunk_id1, block_id1, chunk_id2, block_id2);

        let edge = chunk::Edge {
            chunk_id1,
            block_position1,
            chunk_id2,
            block_position2,
            clearance,
            cost,
        };

        self.edge_map.insert(key, edge);

        for chunk_id in [key.chunk_id1, key.chunk_id2] {
            self.node_edge_map
                .entry(chunk_id)
                .or_insert_with(HashSet::new)
                .insert(key);
        }
    }

    pub fn get_edge_iter(&self, chunk_id: chunk::ID) -> impl Iterator<Item = &chunk::Edge> {
        self.node_edge_map
            .get(&chunk_id)
            .into_iter()
            .flat_map(|edge_set| {
                edge_set
                    .iter()
                    .filter_map(|block_id| self.edge_map.get(block_id))
            })
    }
}
