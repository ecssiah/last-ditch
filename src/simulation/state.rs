use super::chunk::Chunk;
use glam::{Quat, Vec3};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct State {
    pub entity: Arc<RwLock<Entity>>,
    pub world: Arc<RwLock<World>>,
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub id: u32,
    pub name: String,
    pub position: Vec3,
    pub speed: f32,
    pub strafe_speed: f32,
    pub angular_speed: f32,
    pub move_yaw: f32,
    pub look_pitch: f32,
    pub look_yaw: f32,
    pub look_rotation: Quat,
}

#[derive(Debug, Clone)]
pub struct World {
    pub active: bool,
    pub seed: u64,
    pub time: f32,
    pub chunks: Vec<Chunk>,
}
