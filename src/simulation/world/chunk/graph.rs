use glam::IVec3;

use crate::simulation::world::block;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct Graph {
    pub node_map: HashMap<block::ID, block::Node>,
    pub edge_map: HashMap<block::edge::Key, block::Edge>,
    pub node_edge_map: HashMap<block::ID, HashSet<block::edge::Key>>,
}

impl Graph {
    pub fn new() -> Graph {
        let graph = Graph {
            node_map: HashMap::new(),
            edge_map: HashMap::new(),
            node_edge_map: HashMap::new(),
        };

        graph
    }

    pub fn get_block_node_id_list(&self) -> impl Iterator<Item = block::ID> + '_ {
        self.node_map.keys().copied()
    }

    pub fn add_block_node(&mut self, block_node: block::Node) -> Option<block::Node> {
        self.node_map.insert(block_node.block_id, block_node)
    }

    pub fn get_block_node(&self, block_id: block::ID) -> Option<&block::Node> {
        self.node_map.get(&block_id)
    }

    pub fn get_block_node_mut(&mut self, block_id: block::ID) -> Option<&mut block::Node> {
        self.node_map.get_mut(&block_id)
    }

    pub fn has_edge(&self, block_id1: block::ID, block_id2: block::ID) -> bool {
        let (block_id1, block_id2) = if block_id1 < block_id2 {
            (block_id1, block_id2)
        } else {
            (block_id2, block_id1)
        };

        let edge_key = block::edge::Key::new(block_id1, block_id2);

        self.edge_map.contains_key(&edge_key)
    }

    pub fn add_edge(
        &mut self,
        block_id1: block::ID,
        block_position1: IVec3,
        block_id2: block::ID,
        block_position2: IVec3,
        clearance: u32,
        cost: f32,
    ) {
        let ((block_id1, block_position1), (block_id2, block_position2)) = if block_id1 < block_id2
        {
            ((block_id1, block_position1), (block_id2, block_position2))
        } else {
            ((block_id2, block_position2), (block_id1, block_position1))
        };

        let key = block::edge::Key::new(block_id1, block_id2);

        let edge = block::Edge {
            block_id1,
            block_position1,
            block_id2,
            block_position2,
            clearance,
            cost,
        };

        self.edge_map.insert(key, edge);

        for block_id in [key.block_id1, key.block_id2] {
            self.node_edge_map
                .entry(block_id)
                .or_insert_with(HashSet::new)
                .insert(key);
        }
    }

    pub fn get_edge(&self, block_id1: block::ID, block_id2: block::ID) -> Option<&block::Edge> {
        let (block_id1, block_id2) = if block_id1 < block_id2 {
            (block_id1, block_id2)
        } else {
            (block_id2, block_id1)
        };

        let edge_key = block::edge::Key::new(block_id1, block_id2);

        self.edge_map.get(&edge_key)
    }

    pub fn get_edge_iter(&self, block_id: block::ID) -> impl Iterator<Item = &block::Edge> {
        self.node_edge_map
            .get(&block_id)
            .into_iter()
            .flat_map(|edge_set| {
                edge_set
                    .iter()
                    .filter_map(|block_id| self.edge_map.get(block_id))
            })
    }
}
