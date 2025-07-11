use crate::simulation::state::world::graph::{Edge, Graph, Node};
use glam::IVec3;
use std::collections::HashMap;

#[derive(Clone, Debug)]
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

    pub fn get_node(position: IVec3, level: &Level) -> Option<&Node> {
        let region_position =
            Graph::get_region_position(position, level.region_size, level.world_limit);

        let node_map = level.region_node_map.get(&region_position)?;

        node_map.get(&position)
    }

    pub fn get_edge(
        position1: IVec3,
        position2: IVec3,
        edge_map: &HashMap<(IVec3, IVec3), Edge>,
    ) -> Option<&Edge> {
        edge_map.get(&(position1, position2))
    }

    pub fn attach_node(node: Node, region_node_map: &mut HashMap<IVec3, HashMap<IVec3, Node>>) {
        let node_map = region_node_map
            .entry(node.region_position)
            .or_insert_with(HashMap::new);

        node_map.insert(node.position, node);
    }

    pub fn attach_edge(edge: Edge, edge_map: &mut HashMap<(IVec3, IVec3), Edge>) {
        edge_map.insert((edge.node1.position, edge.node2.position), edge);
    }

    pub fn register_search_node(node: Node, search_node_key_vec: &mut Vec<(IVec3, IVec3)>) {
        search_node_key_vec.push((node.region_position, node.position));
    }

    pub fn register_search_edge(edge: Edge, search_edge_key_vec: &mut Vec<(IVec3, IVec3)>) {
        search_edge_key_vec.push((edge.node1.position, edge.node2.position));
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
}
