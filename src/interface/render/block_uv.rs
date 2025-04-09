use crate::simulation::block;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct BlockUV {
    pub kind: block::Kind,
    pub tile_position: HashMap<block::Direction, [u32; 2]>,
}
