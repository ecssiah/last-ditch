use crate::simulation;

#[derive(Clone, Copy)]
pub enum Kind {
    Main,
    Empty,
    WorldTest,
    GraphTest,
    Placeholder,
}

impl Kind {
    pub fn config(&self) -> simulation::Config {
        match self {
            Kind::Placeholder => simulation::Config {
                kind: *self,
                world_radius_in_sectors: 0,
                sector_radius_in_cells: 0,
                seed: 0,
            },
            Kind::Main => simulation::Config {
                kind: *self,
                world_radius_in_sectors: 4,
                sector_radius_in_cells: 8,
                seed: 0,
            },
            Kind::Empty => simulation::Config {
                kind: *self,
                world_radius_in_sectors: 1,
                sector_radius_in_cells: 2,
                seed: 0,
            },
            Kind::WorldTest => simulation::Config {
                kind: *self,
                world_radius_in_sectors: 3,
                sector_radius_in_cells: 4,
                seed: 0,
            },
            Kind::GraphTest => simulation::Config {
                kind: *self,
                world_radius_in_sectors: 2,
                sector_radius_in_cells: 4,
                seed: 0,
            },
        }
    }
}
