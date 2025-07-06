pub mod block_path_task;
pub mod kind;
pub mod node_path_task;

pub use block_path_task::BlockPathTask;
pub use kind::Kind;
pub use node_path_task::NodePathTask;

use crate::simulation::state::{population::Population, world::World};

pub trait Task: Send + Sync + 'static {
    type Snapshot: Send + Sync + 'static;
    type Result: Send + 'static;

    fn snapshot(&self, world: &World, population: &Population) -> Self::Snapshot;
    fn execute(self, snapshot: Self::Snapshot) -> Self::Result;
}
