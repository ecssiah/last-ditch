use crate::simulation::world::block;
use std::f32::EPSILON;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct EdgeKey {
    pub block_id1: block::ID,
    pub block_id2: block::ID,
}

impl EdgeKey {
    pub fn new(block_id1: block::ID, block_id2: block::ID) -> Self {
        assert!(
            block_id1 != block_id2,
            "block edges must travel between different blocks"
        );

        Self {
            block_id1: block_id1.min(block_id2),
            block_id2: block_id2.max(block_id1),
        }
    }
}

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
