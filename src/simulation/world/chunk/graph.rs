use crate::simulation::world::block;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct Graph {
    pub node_map: HashMap<block::ID, block::Node>,
    pub edge_set: HashSet<(block::ID, block::ID)>,
    pub edge_map: HashMap<(block::ID, block::ID), block::Edge>,
}

impl Graph {
    pub fn new() -> Graph {
        let graph = Graph {
            node_map: HashMap::new(),
            edge_set: HashSet::new(),
            edge_map: HashMap::new(),
        };

        graph
    }

    pub fn get_block_ids(&self) -> impl Iterator<Item = &block::ID> {
        self.node_map.keys()
    }

    pub fn add_block_node(
        &mut self,
        block_id: block::ID,
        block_node: block::Node,
    ) -> Option<block::Node> {
        self.node_map.insert(block_id, block_node)
    }

    pub fn get_block_node(&self, block_id: block::ID) -> Option<&block::Node> {
        self.node_map.get(&block_id)
    }

    pub fn get_block_node_mut(&mut self, block_id: block::ID) -> Option<&mut block::Node> {
        self.node_map.get_mut(&block_id)
    }
}
