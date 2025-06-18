use crate::simulation::state::compute::task::{
    chunk_path_task::ChunkPathTask, world_path_task::WorldPathTask,
};

pub enum Kind {
    WorldPath(WorldPathTask),
    ChunkPath(ChunkPathTask),
}
