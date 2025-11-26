use crate::simulation::state;

#[derive(Clone, Copy)]
pub enum Template {
    Placeholder,
    Empty,
    Main,
    Test,
}

impl Template {
    pub fn config(&self) -> state::Config {
        match self {
            state::Template::Placeholder => state::Config {
                seed: 0,
                kind: state::Template::Placeholder,
                sector_radius_in_cells: 0,
                world_radius_in_sectors: 0,
            },
            state::Template::Empty => state::Config {
                seed: 0,
                kind: state::Template::Empty,
                sector_radius_in_cells: 2,
                world_radius_in_sectors: 1,
            },
            state::Template::Main => state::Config {
                seed: 0,
                kind: state::Template::Main,
                sector_radius_in_cells: 8,
                world_radius_in_sectors: 4,
            },
            state::Template::Test => state::Config {
                seed: 0,
                kind: state::Template::Test,
                sector_radius_in_cells: 4,
                world_radius_in_sectors: 3,
            },
        }
    }
}
