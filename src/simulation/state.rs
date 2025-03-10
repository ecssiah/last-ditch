use std::sync::{Arc, RwLock};
use super::{agent::Agent, block::Block, chunk::Chunk, world::World};

#[derive(Debug)]
pub struct State {
    pub agent: Arc<RwLock<Agent>>,
    pub blocks: Arc<[Block]>,
    pub world: Arc<RwLock<World>>,
    pub chunks: Arc<[Arc<RwLock<Chunk>>]>,
}