use crate::simulation::state::world::graph::{Edge, Node};
use glam::IVec3;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Level {
    pub depth: usize,
    pub region_size: usize,
    pub region_node_map: HashMap<IVec3, HashMap<IVec3, Node>>,
    pub edge_map: HashMap<(IVec3, IVec3), Edge>,
    pub search_node_key_vec: Vec<IVec3>,
    pub search_edge_key_vec: Vec<(IVec3, IVec3)>,
}

impl Level {
    pub fn new(depth: usize, region_size: usize) -> Self {
        Self {
            depth,
            region_size,
            region_node_map: HashMap::new(),
            edge_map: HashMap::new(),
            search_node_key_vec: Vec::new(),
            search_edge_key_vec: Vec::new(),
        }
    }

    pub fn neighbors(&self, position: IVec3) -> Vec<Node> {
        let mut node_vec = Vec::new();

        for ((position1, position2), edge) in &self.edge_map {
            if &position == position1 {
                node_vec.push(edge.node2);
            } else if &position == position2 {
                node_vec.push(edge.node1);
            }
        }

        node_vec
    }

    // pub fn get_region_node_vec(&self, region_id: u32) -> Vec<&Node> {
    //     self.node_map
    //         .iter()
    //         .filter(|(_, node)| node.region_id == region_id)
    //         .map(|(_, node)| node)
    //         .collect()
    // }

    // pub fn add_search_node(&mut self, node: Node) {
    //     let node_key = node.position;

    //     self.node_map.insert(node_key, node);
    //     self.search_node_key_vec.push(node_key);
    // }

    // pub fn add_search_edge(&mut self, edge: Edge) {
    //     let edge_key = (edge.node1.position, edge.node2.position);

    //     self.edge_map.insert(edge_key, edge);
    //     self.search_edge_key_vec.push(edge_key);
    // }

    // pub fn reset(&mut self) {
    //     for search_node_key in self.search_node_key_vec.drain(..) {
    //         self.node_map.remove(&search_node_key);
    //     }

    //     for search_edge_key in self.search_edge_key_vec.drain(..) {
    //         self.edge_map.remove(&search_edge_key);
    //     }
    // }
}
