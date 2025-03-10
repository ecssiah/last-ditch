use super::block::Kind;
use glam::{IVec3, Vec3};

#[derive(Debug)]
pub struct Chunk {
    pub modified: bool,
    pub local_position: IVec3,
    pub world_position: Vec3,
    pub palette: Vec<Kind>,
    pub blocks: Box<[u32]>,
}
