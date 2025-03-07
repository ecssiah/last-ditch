use cgmath::Vector3;

use super::chunk::Chunk;
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
    pub position: Vector3<f32>,
    pub direction: Vector3<f32>,
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
