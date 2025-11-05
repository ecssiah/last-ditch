use crate::simulation::state::{
    population::entity::{self},
    world::sector,
};
use glam::{IVec3, Quat, Vec3};

#[derive(Clone, Debug)]
pub struct JudgeView {
    pub entity_id: entity::ID,
    pub position: IVec3,
    pub world_position: Vec3,
    pub sector_id: sector::ID,
    pub sector_coordinates: IVec3,
    pub size: Vec3,
    pub quaternion: Quat,
    pub eye: Vec3,
    pub view_ray_vec: Vec<Vec3>,
}

impl JudgeView {
    pub fn new() -> Self {
        Self {
            entity_id: entity::ID::MAX,
            position: IVec3::ZERO,
            world_position: Vec3::ZERO,
            sector_id: sector::ID::MAX,
            sector_coordinates: IVec3::ZERO,
            size: Vec3::ZERO,
            quaternion: Quat::IDENTITY,
            eye: Vec3::ZERO,
            view_ray_vec: Vec::new(),
        }
    }
}
