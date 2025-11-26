use crate::simulation::state;

#[derive(Clone, Copy)]
pub struct Config {
    pub seed: u64,
    pub kind: state::Template,
    pub sector_radius_in_cells: u32,
    pub world_radius_in_sectors: u32,
}
