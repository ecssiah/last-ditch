use crate::simulation::state::world::sector;
use ultraviolet::{IVec3, Rotor3, Vec3};

#[derive(Clone, Debug)]
pub struct JudgeView {
    pub position: IVec3,
    pub world_position: Vec3,
    pub sector_id: sector::ID,
    pub sector_coordinates: IVec3,
    pub size: Vec3,
    pub rotor: Rotor3,
    pub eye: Vec3,
}

impl JudgeView {
    pub fn new() -> Self {
        Self {
            position: IVec3::new(0, 0, 0),
            world_position: Vec3::broadcast(0.0),
            sector_id: sector::ID(0),
            sector_coordinates: IVec3::new(0, 0, 0),
            size: Vec3::broadcast(0.0),
            rotor: Rotor3::identity(),
            eye: Vec3::broadcast(0.0),
        }
    }
}
