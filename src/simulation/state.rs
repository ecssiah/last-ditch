use super::chunk::Chunk;
use glam::{Quat, Vec3};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct State {
    pub judge: Arc<RwLock<Judge>>,
    pub entities: Arc<RwLock<Entities>>,
    pub world: Arc<RwLock<World>>,
}

#[derive(Debug, Clone)]
pub struct Judge {
    pub name: String,
    pub position: Vec3,
    pub speed: f32,
    pub strafe_speed: f32,
    pub angular_speed: f32,
    pub rotation: Quat,
}

#[derive(Debug, Clone)]
pub struct Entities {}

#[derive(Debug, Clone)]
pub struct World {
    pub active: bool,
    pub seed: u64,
    pub time: f32,
    pub chunks: Vec<Chunk>,
}
