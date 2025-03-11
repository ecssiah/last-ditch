use super::{agent::Agent, block::Block, chunk::Chunk, world::World};
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct State {
    pub agent: Arc<RwLock<Agent>>,
    pub blocks: Arc<Vec<Block>>,
    pub world: Arc<RwLock<World>>,
    pub chunks: Arc<[Arc<RwLock<Chunk>>]>,
}
