use crate::simulation::world::block;
use glam::IVec3;

#[derive(Clone, Debug)]
pub struct Node {
    pub block_id: block::ID,
    pub position: IVec3,
    pub group_id: u32,
}
