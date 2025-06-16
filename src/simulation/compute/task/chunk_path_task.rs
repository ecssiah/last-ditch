use crate::simulation::{
    compute::{
        result::chunk_path_result::ChunkPathResult,
        snapshot::chunk_path_snapshot::ChunkPathSnapshot, Task,
    },
    population::{agent, Population},
    world::{block, chunk, World},
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

    fn snapshot(&self, world: &World, _population: &Population) -> Self::Snapshot {
        let chunk_graph = world
            .graph
            .get_chunk_graph(self.chunk_id)
            .cloned()
            .unwrap_or(chunk::Graph::new());

        Self::Snapshot { chunk_graph }
    }

    fn execute(self, snapshot: Self::Snapshot) -> Self::Result {
        Self::Result {
            agent_id: self.agent_id,
            path: Vec::new(),
        }
    }
}
