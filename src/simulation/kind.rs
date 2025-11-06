use crate::simulation;

#[derive(Clone, Copy)]
pub enum Kind {
    Placeholder,
    EmptyWorld,
    MainWorld,
    TestWorld,
    GraphWorld,
}

impl Kind {
    pub fn config(&self) -> simulation::Config {
        match self {
            Kind::Placeholder => simulation::Config {
                seed: 0,
                kind: *self,
                sector_radius_in_cells: 0,
                world_radius_in_sectors: 0,
            },
            Kind::EmptyWorld => simulation::Config {
                seed: 0,
                kind: *self,
                sector_radius_in_cells: 2,
                world_radius_in_sectors: 1,
            },
            Kind::MainWorld => simulation::Config {
                seed: 0,
                kind: *self,
                sector_radius_in_cells: 8,
                world_radius_in_sectors: 4,
            },
            Kind::TestWorld => simulation::Config {
                seed: 0,
                kind: *self,
                sector_radius_in_cells: 4,
                world_radius_in_sectors: 3,
            },
            Kind::GraphWorld => simulation::Config {
                seed: 0,
                kind: *self,
                sector_radius_in_cells: 4,
                world_radius_in_sectors: 2,
            },
        }
    }
}
