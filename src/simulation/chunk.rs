use super::block::Kind;
use glam::IVec3;

#[derive(Debug)]
pub struct Chunk {
    pub modified: bool,
    pub position: IVec3,
    pub palette: Vec<Kind>,
    pub blocks: Box<[u32]>,
}
