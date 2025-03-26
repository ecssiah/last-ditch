use crate::simulation::{agent::Agent, chunk::Chunk, physics::Physics, world::World};
use std::sync::{Arc, RwLock};

pub struct State {
    pub agent: Arc<RwLock<Agent>>,
    pub world: Arc<RwLock<World>>,
    pub physics: Arc<RwLock<Physics>>,
    pub chunks: Arc<[Arc<RwLock<Chunk>>]>,
}
