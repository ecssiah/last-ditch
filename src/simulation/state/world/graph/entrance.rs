use crate::simulation::state::world::graph::Transition;
use glam::IVec3;

pub struct Entrance {
    pub region1_position: IVec3,
    pub region2_position: IVec3,
    pub transitions: Vec<Transition>,
}

impl Entrance {
    pub fn new(region1_position: IVec3, region2_position: IVec3) -> Self {
        Self {
            region1_position,
            region2_position,
            transitions: Vec::new(),
        }
    }
}
