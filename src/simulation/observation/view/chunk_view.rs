use crate::simulation::state::world::{block::Block, chunk};

#[derive(Clone, Debug)]
pub struct ChunkView {
    pub id: chunk::ID,
    pub block_vec: Vec<Block>,
}

impl ChunkView {
    pub fn new() -> Self {
        Self {
            id: chunk::ID::MAX,
            block_vec: Vec::new(),
        }
    }
}
