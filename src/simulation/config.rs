use crate::simulation;

#[derive(Clone, Copy)]
pub struct Config {
    pub seed: u64,
    pub kind: simulation::Kind,
    pub sector_radius_in_cells: u32,
    pub world_radius_in_sectors: u32,
}
