use crate::simulation::world;
use glam::IVec3;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Node {
    pub(crate) edge_map: HashMap<IVec3, Vec<world::Edge>>,
    pub(crate) group_id: u32,
}

impl Node {
    pub fn new() -> Self {
        let node = Self {
            edge_map: HashMap::new(),
            group_id: 0,
        };

        node
    }
}
