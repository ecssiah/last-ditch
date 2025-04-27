use crate::simulation::{self};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct BlockData {
    pub kind: simulation::world::block::Kind,
    pub tile_position_map: HashMap<simulation::world::grid::Direction, [u32; 2]>,
}
