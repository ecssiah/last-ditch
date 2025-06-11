use crate::simulation::world::chunk;
use glam::IVec3;

#[derive(Clone, Debug)]
pub struct Node {
    pub chunk_id: chunk::ID,
    pub position: IVec3,
    pub group_id: u32,
}
