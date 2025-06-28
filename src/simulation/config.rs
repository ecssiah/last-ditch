use crate::simulation;

#[derive(Clone, Copy)]
pub struct Config {
    pub kind: simulation::Kind,
    pub world_radius: u32,
    pub chunk_radius: u32,
    pub seed: u64,
}
