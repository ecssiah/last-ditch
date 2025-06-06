use crate::simulation::world::chunk;
use glam::IVec3;

#[derive(Clone, Debug)]
pub struct Node {
    pub(crate) grid_position: IVec3,
    pub(crate) edge_list: Vec<chunk::Edge>,
    pub(crate) clearance: u32,
    pub(crate) group_id: u32,
}
