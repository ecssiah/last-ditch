use crate::simulation::state::world::{block, sector};
use ultraviolet::{IVec3, Rotor3, Vec3};

#[derive(Clone, Debug)]
pub struct JudgeView {
    pub position: IVec3,
    pub world_position: Vec3,
    pub sector_id: sector::ID,
    pub sector_coordinates: IVec3,
    pub size: Vec3,
    pub sight_world_position: Vec3,
    pub sight_rotor: Rotor3,
    pub selected_block_kind: block::Kind,
}

impl JudgeView {
    pub fn new() -> Self {
        Self {
            position: IVec3::new(0, 0, 0),
            world_position: Vec3::broadcast(0.0),
            sector_id: sector::ID(0),
            sector_coordinates: IVec3::new(0, 0, 0),
            size: Vec3::broadcast(0.0),
            sight_world_position: Vec3::broadcast(0.0),
            sight_rotor: Rotor3::identity(),
            selected_block_kind: block::Kind::Engraved1,
        }
    }
}
