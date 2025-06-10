use crate::simulation::world::block;
use crate::simulation::world::chunk;
use glam::IVec3;
use std::f32::EPSILON;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct EdgeKey {
    pub chunk_id1: chunk::ID,
    pub block_id1: block::ID,
    pub chunk_id2: chunk::ID,
    pub block_id2: block::ID,
}

impl EdgeKey {
    pub fn new(
        chunk_id1: chunk::ID,
        block_id1: block::ID,
        chunk_id2: chunk::ID,
        block_id2: block::ID,
    ) -> Self {
        assert!(
            chunk_id1 != chunk_id2,
            "chunk edges must travel between different chunks"
        );

        if chunk_id1 < chunk_id2 {
            Self {
                chunk_id1: chunk_id1,
                block_id1: block_id1,
                chunk_id2: chunk_id2,
                block_id2: block_id2,
            }
        } else {
            Self {
                chunk_id1: chunk_id2,
                block_id1: block_id2,
                chunk_id2: chunk_id1,
                block_id2: block_id1,
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Edge {
    pub(crate) chunk_id1: chunk::ID,
    pub(crate) chunk_id2: chunk::ID,
    pub(crate) position1: IVec3,
    pub(crate) position2: IVec3,
    pub(crate) clearance: u32,
    pub(crate) cost: f32,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.chunk_id1 == other.chunk_id1
            && self.chunk_id2 == other.chunk_id2
            && self.position1 == other.position1
            && self.position2 == other.position2
            && self.clearance == other.clearance
            && (self.cost - other.cost).abs() < EPSILON
    }
}

impl Eq for Edge {}
