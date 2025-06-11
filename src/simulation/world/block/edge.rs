use glam::IVec3;

use crate::simulation::world::block;
use std::f32::EPSILON;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Key {
    pub block_id1: block::ID,
    pub block_id2: block::ID,
}

impl Key {
    pub fn new(block_id1: block::ID, block_id2: block::ID) -> Self {
        assert_ne!(
            block_id1, block_id2,
            "block edges must connect distinct blocks"
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
    pub block_position1: IVec3,
    pub block_id2: block::ID,
    pub block_position2: IVec3,
    pub clearance: u32,
    pub cost: f32,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.block_id1 == other.block_id1
            && self.block_position1 == other.block_position1
            && self.block_id2 == other.block_id2
            && self.block_position2 == other.block_position2
            && self.clearance == other.clearance
            && (self.cost - other.cost).abs() < EPSILON
    }
}

impl Eq for Edge {}
