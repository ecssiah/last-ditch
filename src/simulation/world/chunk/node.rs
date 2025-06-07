use crate::simulation::world::chunk;

#[derive(Clone, Debug)]
pub struct Node {
    pub(crate) edge_list: Vec<chunk::Edge>,
    pub(crate) clearance: u32,
    pub(crate) group_id: u32,
}
