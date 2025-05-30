use crate::simulation::world::chunk;
use glam::IVec3;

pub struct Node {
    pub grid_position: IVec3,
    pub clearance: usize,
    pub edge_list: Vec<chunk::Edge>,
}
