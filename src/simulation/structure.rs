use super::block::BlockKind;
use serde::Deserialize;

#[derive(Debug, Deserialize, Hash, Eq, PartialEq)]
pub enum StructureKind {
    Swastika,
    Mario,
    Luigi,
}

#[derive(Debug, Deserialize)]
pub struct BlockData {
    pub kind: BlockKind,
    pub position: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct Structure {
    pub size: Vec<u32>,
    pub blocks: Vec<BlockData>,
}
