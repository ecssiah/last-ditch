use crate::simulation::block;
use serde::Deserialize;

#[derive(Debug, Deserialize, Hash, Eq, PartialEq)]
pub enum Kind {
    Mario,
    Luigi,
    LightTest,
}

#[derive(Debug, Deserialize)]
pub struct BlockData {
    pub kind: block::Kind,
    pub position: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct Structure {
    pub kind: Kind,
    pub size: Vec<u32>,
    pub blocks: Vec<BlockData>,
}