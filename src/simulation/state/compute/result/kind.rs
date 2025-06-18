use crate::simulation::state::compute::result::{
    chunk_path_result::ChunkPathResult, world_path_result::WorldPathResult,
};

#[derive(Debug)]
pub enum Kind {
    WorldPath(WorldPathResult),
    ChunkPath(ChunkPathResult),
}
