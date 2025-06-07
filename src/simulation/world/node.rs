use crate::simulation::world::{self};
use glam::IVec3;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Node {
    pub(crate) edge_map: HashMap<IVec3, Vec<world::Edge>>,
}

impl Node {
    pub fn new() -> Self {
        let node = Self {
            edge_map: HashMap::new(),
        };

        node
    }

    pub fn get_edge(
        &self,
        from_position: IVec3,
        to_position: IVec3,
    ) -> Option<world::Edge> {
        let (_, edge_list) = self
            .edge_map
            .iter()
            .find(|(position, _)| **position == from_position)?;

        edge_list
            .iter()
            .find(|edge| edge.to_position == to_position)
            .cloned()
    }
}
