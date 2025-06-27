use crate::simulation;

#[derive(Clone, Copy)]
pub struct Config {
    pub mode: simulation::Mode,
    pub world_radius: u32,
    pub chunk_radius: u32,
    pub seed: u64,
}
