use crate::simulation::{compute, population::agent, world::block};

pub struct ChunkPathResult {
    pub agent_id: agent::ID,
    pub path: Option<Vec<block::ID>>,
}

impl compute::Result for ChunkPathResult {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
