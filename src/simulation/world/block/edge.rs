use crate::simulation::world::block;
use std::f32::EPSILON;

#[derive(Clone, Debug)]
pub struct Edge {
    pub block_id1: block::ID,
    pub block_id2: block::ID,
    pub clearance: u32,
    pub cost: f32,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.block_id1 == other.block_id1
            && self.block_id2 == other.block_id2
            && self.clearance == other.clearance
            && (self.cost - other.cost).abs() < EPSILON
    }
}

impl Eq for Edge {}
