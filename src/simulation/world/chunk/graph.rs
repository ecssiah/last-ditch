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

    pub fn add_node(&mut self, position: IVec3, node: chunk::Node) -> Option<chunk::Node> {
        self.node_map.insert(position, node)
    }

    pub fn add_edge(&mut self, position: IVec3, edge: chunk::Edge) {
        if let Some(node) = self.node_map.get_mut(&position) {
            node.edge_list.push(edge);
        }
    }

    pub fn create_edges(
        &mut self,
        from_position: IVec3,
        to_position: IVec3,
        clearance: u32,
        cost: f32,
    ) {
        self.add_edge(
            from_position,
            chunk::Edge {
                from_position: from_position,
                to_position: to_position,
                clearance,
                cost,
            },
        );

        self.add_edge(
            to_position,
            chunk::Edge {
                from_position: to_position,
                to_position: from_position,
                clearance,
                cost,
            },
        );
    }

    pub fn has_node(&self, position: IVec3) -> bool {
        self.node_map.contains_key(&position)
    }

    pub fn get_node(&self, position: IVec3) -> Option<&chunk::Node> {
        self.node_map.get(&position)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&IVec3, &chunk::Node)> {
        self.node_map.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&IVec3, &mut chunk::Node)> {
        self.node_map.iter_mut()
    }
}
