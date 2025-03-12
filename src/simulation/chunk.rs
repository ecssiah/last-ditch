use super::block::BlockKind;
use glam::IVec3;

#[derive(Debug)]
pub struct Chunk {
    pub modified: bool,
    pub position: IVec3,
    pub palette: Vec<BlockKind>,
    pub blocks: Box<[u32]>,
}
