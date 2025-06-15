pub mod chunk_path_task;
pub mod kind;
pub mod world_path_task;

pub use chunk_path_task::ChunkPathTask;
pub use kind::Kind;
pub use world_path_task::WorldPathTask;

use crate::simulation::{population::Population, world::World};

pub trait Task: Send + Sync + 'static {
    type Snapshot: Send + Sync + 'static;
    type Result: Send + 'static;

    fn snapshot(&self, world: &World, population: &Population) -> Self::Snapshot;
    fn execute(self, snapshot: Self::Snapshot) -> Self::Result;
}
