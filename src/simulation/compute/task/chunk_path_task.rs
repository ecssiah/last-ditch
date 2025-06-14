use crate::simulation::{
    population::agent,
    world::{block, chunk},
};

pub struct ChunkPathTask {
    pub agent_id: agent::ID,
    pub chunk_id: chunk::ID,
    pub block_id_from: block::ID,
    pub block_id_to: block::ID,
}
