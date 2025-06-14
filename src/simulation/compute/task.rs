pub mod chunk_path_task;

use std::sync::Arc;

pub use chunk_path_task::ChunkPathTask;

use crate::simulation::{
    compute::{self},
    population::Population,
    world::World,
};

pub trait Task: Send + Sync {
    fn snapshot(
        &self,
        world: &World,
        population: &Population,
    ) -> Box<dyn compute::Snapshot>;

    fn execute(self: Arc<Self>, snapshot: Box<dyn compute::Snapshot>) -> Box<dyn compute::Result>;
}
