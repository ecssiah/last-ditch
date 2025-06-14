use crate::simulation::world::block;
use crate::simulation::world::chunk;
use glam::IVec3;
use std::f32::EPSILON;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Key {
    pub chunk_id1: chunk::ID,
    pub block_id1: block::ID,
    pub chunk_id2: chunk::ID,
    pub block_id2: block::ID,
}

impl Key {
    pub fn new(
        chunk_id1: chunk::ID,
        block_id1: block::ID,
        chunk_id2: chunk::ID,
        block_id2: block::ID,
    ) -> Self {
        assert_ne!(
            chunk_id1, chunk_id2,
            "chunk edges must connect distinct chunks"
        );

        assert_ne!(
            block_id1, block_id2,
            "chunk edges must connect distinct blocks"
        );

        Self {
            chunk_id1: chunk_id1,
            block_id1: block_id1,
            chunk_id2: chunk_id2,
            block_id2: block_id2,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Edge {
    pub chunk_id1: chunk::ID,
    pub block_position1: IVec3,
    pub chunk_id2: chunk::ID,
    pub block_position2: IVec3,
    pub clearance: u32,
    pub cost: f32,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.chunk_id1 == other.chunk_id1
            && self.block_position1 == other.block_position1
            && self.chunk_id2 == other.chunk_id2
            && self.block_position2 == other.block_position2
            && self.clearance == other.clearance
            && (self.cost - other.cost).abs() < EPSILON
    }
}

impl Eq for Edge {}
