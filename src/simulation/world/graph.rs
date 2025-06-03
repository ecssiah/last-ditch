use crate::simulation::world::{self};
use glam::IVec3;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Graph {
    pub node_map: HashMap<IVec3, world::Node>,
}

impl Graph {
    pub fn new() -> Self {
        let graph = Graph {
            node_map: HashMap::new(),
        };

        graph
    }

    pub fn has_node(&self, chunk_position: IVec3) -> bool {
        self.node_map.contains_key(&chunk_position)
    }

    pub fn get_node(&self, chunk_position: IVec3) -> Option<&world::Node> {
        self.node_map.get(&chunk_position)
    }

    pub fn add_node(&mut self, chunk_position: IVec3, node: world::Node) -> Option<world::Node> {
        self.node_map.insert(chunk_position, node)
    }

    pub fn add_edge(&mut self, chunk_position: IVec3, edge: world::Edge) {
        if let Some(node) = self.node_map.get_mut(&chunk_position) {
            if !node.edge_list.contains(&edge) {
                node.edge_list.push(edge);
            }
        }
    }

    pub fn create_edges(
        &mut self,
        chunk_position: IVec3,
        target_chunk_position: IVec3,
        from_grid_position: IVec3,
        to_grid_position: IVec3,
        clearance: i32,
        cost: f32,
    ) {
        let node1 = self.get_node(chunk_position);
        let node2 = self.get_node(target_chunk_position);

        if node1.is_some() && node2.is_some() {
            self.add_edge(
                chunk_position,
                world::Edge {
                    target_chunk_position: target_chunk_position,
                    from_grid_position: from_grid_position,
                    to_grid_position: to_grid_position,
                    clearance: clearance as u32,
                    cost,
                    group_id: 0,
                },
            );

            self.add_edge(
                target_chunk_position,
                world::Edge {
                    target_chunk_position: chunk_position,
                    from_grid_position: to_grid_position,
                    to_grid_position: from_grid_position,
                    clearance: clearance as u32,
                    cost,
                    group_id: 0,
                },
            );
        }
    }
}
