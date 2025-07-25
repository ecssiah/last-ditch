use crate::simulation::state::population::entity;
use glam::Vec3;

pub struct EntityRenderData {
    pub entity_id: entity::ID,
    pub world_position: Vec3,
    pub rotation: f32,
}
