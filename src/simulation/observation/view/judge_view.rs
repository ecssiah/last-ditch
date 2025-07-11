use crate::simulation::state::{
    population::entity::{self},
    world::chunk,
};
use glam::{IVec3, Quat, Vec3};

#[derive(Clone, Debug)]
pub struct JudgeView {
    pub entity_id: entity::ID,
    pub position: IVec3,
    pub world_position: Vec3,
    pub chunk_id: chunk::ID,
    pub chunk_coordinates: IVec3,
    pub size: Vec3,
    pub quaternion: Quat,
}

impl JudgeView {
    pub fn new() -> Self {
        Self {
            entity_id: entity::ID::MAX,
            position: IVec3::ZERO,
            world_position: Vec3::ZERO,
            chunk_id: chunk::ID::MAX,
            chunk_coordinates: IVec3::ZERO,
            size: Vec3::ZERO,
            quaternion: Quat::IDENTITY,
        }
    }
}
