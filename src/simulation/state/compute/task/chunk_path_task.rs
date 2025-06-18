use crate::simulation::{
    state::compute::{
        result::chunk_path_result::ChunkPathResult,
        snapshot::chunk_path_snapshot::ChunkPathSnapshot, Task,
    },
    state::population::{agent, Population},
    state::world::{block, chunk, World},
};

#[derive(Debug)]
pub struct ChunkPathTask {
    pub agent_id: agent::ID,
    pub chunk_id: chunk::ID,
    pub block_id_start: block::ID,
    pub block_id_end: block::ID,
}

impl Task for ChunkPathTask {
    type Snapshot = ChunkPathSnapshot;
    type Result = ChunkPathResult;

    fn snapshot(&self, _world: &World, _population: &Population) -> Self::Snapshot {
        Self::Snapshot {}
    }

    fn execute(self, _snapshot: Self::Snapshot) -> Self::Result {
        Self::Result {
            agent_id: self.agent_id,
            path: Vec::new(),
        }
    }
}
