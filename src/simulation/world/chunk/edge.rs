use glam::IVec3;
use std::f32::EPSILON;

#[derive(Clone, Debug)]
pub struct Edge {
    pub target_grid_position: IVec3,
    pub clearance: i32,
    pub cost: f32,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.target_grid_position == other.target_grid_position
            && self.clearance == other.clearance
            && (self.cost - other.cost).abs() < EPSILON
    }
}

impl Eq for Edge {}
