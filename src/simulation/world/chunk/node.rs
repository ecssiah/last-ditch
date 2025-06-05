use crate::simulation::world::chunk;
use glam::IVec3;

#[derive(Clone, Debug)]
pub struct Node {
    pub grid_position: IVec3,
    pub clearance: u32,
    pub edge_list: Vec<chunk::Edge>,
}
