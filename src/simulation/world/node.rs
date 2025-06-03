use crate::simulation::world;

#[derive(Clone, Debug)]
pub struct Node {
    pub edge_list: Vec<world::Edge>,
}

impl Node {
    pub fn new() -> Self {
        let node = Self {
            edge_list: Vec::new(),
        };

        node
    }
}
