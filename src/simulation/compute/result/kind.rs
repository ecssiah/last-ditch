use crate::simulation::compute::result::{
    chunk_path_result::ChunkPathResult, world_path_result::WorldPathResult,
};

pub enum Kind {
    WorldPath(WorldPathResult),
    ChunkPath(ChunkPathResult),
}
