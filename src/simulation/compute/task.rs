pub mod chunk_path_task;

pub use chunk_path_task::ChunkPathTask;

use crate::simulation::compute::{self};

pub trait Task: Send {
    fn execute(self: Box<Self>) -> Box<dyn compute::Result>;
}
