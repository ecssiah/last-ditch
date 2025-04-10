use crate::simulation::block;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct RenderBlock {
    pub kind: block::Kind,
    pub atlas_coordinates: HashMap<block::Direction, [u32; 2]>,
}
