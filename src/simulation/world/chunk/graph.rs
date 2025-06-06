use crate::simulation::world::chunk::{self};
use glam::IVec3;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Graph {
    pub node_map: HashMap<IVec3, chunk::Node>,
}

impl Graph {
    pub fn new() -> Graph {
        let graph = Graph {
            node_map: HashMap::new(),
        };

        graph
    }

    pub fn add_node(&mut self, grid_position: IVec3, node: chunk::Node) -> Option<chunk::Node> {
        self.node_map.insert(grid_position, node)
    }

    pub fn add_edge(&mut self, grid_position: IVec3, edge: chunk::Edge) {
        if let Some(node) = self.node_map.get_mut(&grid_position) {
            node.edge_list.push(edge);
        }
    }

    pub fn create_edges(
        &mut self,
        from_grid_position: IVec3,
        to_grid_position: IVec3,
        clearance: u32,
        cost: f32,
    ) {
        self.add_edge(
            from_grid_position,
            chunk::Edge {
                from_grid_position: from_grid_position,
                to_grid_position: to_grid_position,
                clearance,
                cost,
            },
        );

        self.add_edge(
            to_grid_position,
            chunk::Edge {
                from_grid_position: to_grid_position,
                to_grid_position: from_grid_position,
                clearance,
                cost,
            },
        );
    }

    pub fn has_node(&self, grid_position: IVec3) -> bool {
        self.node_map.contains_key(&grid_position)
    }

    pub fn get_node(&self, grid_position: IVec3) -> Option<&chunk::Node> {
        self.node_map.get(&grid_position)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&IVec3, &chunk::Node)> {
        self.node_map.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&IVec3, &mut chunk::Node)> {
        self.node_map.iter_mut()
    }
}
