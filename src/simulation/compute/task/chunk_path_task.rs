use crate::simulation::{
    compute::{self},
    population::agent,
    world::{block, chunk},
};
use std::sync::Arc;

pub struct ChunkPathTask {
    pub agent_id: agent::ID,
    pub chunk_id: chunk::ID,
    pub block_id_from: block::ID,
    pub block_id_to: block::ID,
}

impl compute::Task for ChunkPathTask {
    fn snapshot(
        &self,
        world: &crate::simulation::world::World,
        population: &crate::simulation::population::Population,
    ) -> Box<dyn compute::Snapshot> {
        println!("I'm doing a Snapshot!");

        Box::new(compute::snapshot::ChunkPathSnapshot {})
    }

    fn execute(
        self: Arc<Self>,
        snapshot: Box<dyn compute::Snapshot>,
    ) -> Box<dyn crate::simulation::compute::Result> {
        println!("I'm doing an EXECUTE!");

        let result = compute::result::ChunkPathResult {
            agent_id: agent::ID(0),
            path: Some(Vec::new()),
        };

        Box::new(result)
    }
}
