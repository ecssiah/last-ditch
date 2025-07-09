use crate::simulation::state::world::graph::{Edge, Node};
use glam::IVec3;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Level {
    pub depth: usize,
    pub world_limit: usize,
    pub region_size: usize,
    pub region_node_map: HashMap<IVec3, HashMap<IVec3, Node>>,
    pub edge_map: HashMap<(IVec3, IVec3), Edge>,
    pub search_node_keys_vec: Vec<(IVec3, IVec3)>,
    pub search_edge_key_vec: Vec<(IVec3, IVec3)>,
}

impl Level {
    pub fn new(depth: usize, region_size: usize, world_limit: usize) -> Self {
        Self {
            depth,
            world_limit,
            region_size,
            region_node_map: HashMap::new(),
            edge_map: HashMap::new(),
            search_node_keys_vec: Vec::new(),
            search_edge_key_vec: Vec::new(),
        }
    }

    pub fn edge_vec(position: IVec3, edge_map: &HashMap<(IVec3, IVec3), Edge>) -> Vec<Edge> {
        edge_map
            .iter()
            .filter_map(|(&(position1, position2), &edge)| {
                if position1 == position || position2 == position {
                    Some(edge)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn neighbor_vec(position: IVec3, edge_map: &HashMap<(IVec3, IVec3), Edge>) -> Vec<Node> {
        edge_map
            .iter()
            .filter_map(|(&(position1, position2), &edge)| {
                if position == position1 {
                    Some(edge.node2)
                } else if position == position2 {
                    Some(edge.node1)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn add_search_node(&mut self, node: Node) {
        let node_map = self
            .region_node_map
            .entry(node.region_position)
            .or_insert_with(HashMap::new);

        node_map.insert(node.position, node);

        self.search_node_keys_vec
            .push((node.region_position, node.position));
    }

    pub fn add_search_edge(&mut self, edge: Edge) {
        let edge_key = (edge.node1.position, edge.node2.position);

        self.edge_map.insert(edge_key, edge);
        self.search_edge_key_vec.push(edge_key);
    }

    pub fn reset(&mut self) {
        for (region_position, node_position) in self.search_node_keys_vec.drain(..) {
            if let Some(node_map) = self.region_node_map.get_mut(&region_position) {
                node_map.remove(&node_position);
            }
        }

        for search_edge_key in self.search_edge_key_vec.drain(..) {
            self.edge_map.remove(&search_edge_key);
        }
    }
}
