use crate::simulation;

#[derive(Clone, Copy)]
pub enum Kind {
    Main,
    Placeholder,
    Empty,
    WorldTest,
    GraphTest,
}

impl Kind {
    pub fn config(&self) -> simulation::Config {
        match self {
            Kind::Main => simulation::Config {
                kind: *self,
                world_radius: 4,
                chunk_radius: 8,
                seed: 0,
            },
            Kind::Placeholder => simulation::Config {
                kind: *self,
                world_radius: 0,
                chunk_radius: 0,
                seed: 0,
            },
            Kind::Empty => simulation::Config {
                kind: *self,
                world_radius: 1,
                chunk_radius: 2,
                seed: 0,
            },
            Kind::WorldTest => simulation::Config {
                kind: *self,
                world_radius: 3,
                chunk_radius: 4,
                seed: 0,
            },
            Kind::GraphTest => simulation::Config {
                kind: *self,
                world_radius: 3,
                chunk_radius: 4,
                seed: 0,
            },
        }
    }
}
