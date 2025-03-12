use super::block::Kind;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BlockData {
    pub kind: Kind,
    pub position: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct Structure {
    pub size: Vec<u32>,
    pub blocks: Vec<BlockData>,
}
