use glam::IVec3;
use std::f32::EPSILON;

#[derive(Clone, Debug)]
pub struct Edge {
    pub from_position: IVec3,
    pub to_position: IVec3,
    pub clearance: u32,
    pub cost: f32,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.to_position == other.to_position
            && self.from_position == other.from_position
            && self.clearance == other.clearance
            && (self.cost - other.cost).abs() < EPSILON
    }
}

impl Eq for Edge {}
