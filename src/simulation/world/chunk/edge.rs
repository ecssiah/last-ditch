use crate::simulation::world::chunk;
use glam::IVec3;
use std::f32::EPSILON;

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
