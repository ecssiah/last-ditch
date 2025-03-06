use super::chunk::Chunk;
use crate::consts::WORLD_VOLUME;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct State {
    pub leader: Arc<RwLock<Leader>>,
    pub entities: Arc<RwLock<Entities>>,
    pub world: Arc<RwLock<World>>,
}

#[derive(Debug, Clone)]
pub struct Leader {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Entities {}

#[derive(Debug, Clone)]
pub struct World {
    pub active: bool,
    pub seed: u64,
    pub time: f64,
    pub chunks: Vec<Chunk>,
}
