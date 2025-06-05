use glam::IVec3;
use std::f32::EPSILON;

#[derive(Clone, Debug)]
pub struct Edge {
    pub from_chunk_position: IVec3,
    pub to_chunk_position: IVec3,
    pub from_grid_position: IVec3,
    pub to_grid_position: IVec3,
    pub clearance: u32,
    pub cost: f32,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.from_chunk_position == other.from_chunk_position
            && self.to_chunk_position == other.to_chunk_position
            && self.from_grid_position == other.from_grid_position
            && self.to_grid_position == other.to_grid_position
            && self.clearance == other.clearance
            && (self.cost - other.cost).abs() < EPSILON
    }
}

impl Eq for Edge {}
