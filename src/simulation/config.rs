use crate::simulation;

#[derive(Clone, Copy)]
pub struct Config {
    pub kind: simulation::Kind,
    pub world_extent_chunks: u32,
    pub chunk_extent_blocks: u32,
    pub seed: u64,
}
