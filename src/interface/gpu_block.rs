use crate::simulation::{self};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct GPUBlock {
    pub kind: simulation::block::Kind,
    pub atlas_coordinates: HashMap<simulation::block::Direction, [u32; 2]>,
}
