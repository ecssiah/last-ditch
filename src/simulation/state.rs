use crate::simulation::{self, agent::Agent, world::World};
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct State {
    pub agent: Arc<RwLock<Agent>>,
    pub world: Arc<RwLock<World>>,
    pub chunks: Arc<[Arc<RwLock<simulation::Chunk>>]>,
}