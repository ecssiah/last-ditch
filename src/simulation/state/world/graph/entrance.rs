use crate::simulation::state::world::graph::Transition;
use glam::IVec3;

pub struct Entrance {
    pub region1_coordinates: IVec3,
    pub region2_coordinates: IVec3,
    pub transitions: Vec<Transition>,
}

impl Entrance {
    pub fn new(region1_coordinates: IVec3, region2_coordinates: IVec3) -> Self {
        Self {
            region1_coordinates,
            region2_coordinates,
            transitions: Vec::new(),
        }
    }
}
