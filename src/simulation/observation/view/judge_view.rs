use crate::simulation::state::{
    population::entity::{self},
    world::chunk,
};
use glam::{IVec3, Quat, Vec3};

#[derive(Clone, Default, Debug)]
pub struct JudgeView {
    pub id: entity::ID,
    pub position: IVec3,
    pub world_position: Vec3,
    pub chunk_id: chunk::ID,
    pub chunk_coordinates: IVec3,
    pub size: Vec3,
    pub quaternion: Quat,
}
