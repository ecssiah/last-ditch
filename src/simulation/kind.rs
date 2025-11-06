use crate::simulation;

#[derive(Clone, Copy)]
pub enum Kind {
    Placeholder,
    Empty,
    Main,
    Test,
    Graph,
}

impl Kind {
    pub fn config(&self) -> simulation::Config {
        match self {
            Kind::Placeholder => simulation::Config {
                seed: 0,
                simulation_kind: Kind::Placeholder,
                sector_radius_in_cells: 0,
                world_radius_in_sectors: 0,
            },
            Kind::Empty => simulation::Config {
                seed: 0,
                simulation_kind: Kind::Empty,
                sector_radius_in_cells: 2,
                world_radius_in_sectors: 1,
            },
            Kind::Main => simulation::Config {
                seed: 0,
                simulation_kind: Kind::Main,
                sector_radius_in_cells: 8,
                world_radius_in_sectors: 4,
            },
            Kind::Test => simulation::Config {
                seed: 0,
                simulation_kind: Kind::Test,
                sector_radius_in_cells: 4,
                world_radius_in_sectors: 3,
            },
            Kind::Graph => simulation::Config {
                seed: 0,
                simulation_kind: Kind::Graph,
                sector_radius_in_cells: 4,
                world_radius_in_sectors: 2,
            },
        }
    }
}
