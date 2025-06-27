use crate::simulation::state::world::graph::Transition;
use glam::IVec3;
use std::fmt;

pub struct Entrance {
    pub region1_coordinates: IVec3,
    pub region2_coordinates: IVec3,
    pub transition_vec: Vec<Transition>,
}

impl Entrance {
    pub fn new(region1_coordinates: IVec3, region2_coordinates: IVec3) -> Self {
        Self {
            region1_coordinates,
            region2_coordinates,
            transition_vec: Vec::new(),
        }
    }
}

impl fmt::Debug for Entrance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Entrance")
            .field("region1_coordinates", &self.region1_coordinates)
            .field("region2_coordinates", &self.region2_coordinates)
            .field("transition count", &self.transition_vec.len())
            .finish()
    }
}
