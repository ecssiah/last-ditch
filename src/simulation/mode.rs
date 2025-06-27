use crate::simulation;

#[derive(Clone, Copy)]
pub enum Mode {
    Main,
    Empty,
    WorldTest,
    GraphTest,
}

impl Mode {
    pub fn config(&self) -> simulation::Config {
        match self {
            Mode::Main => simulation::Config {
                mode: *self,
                world_radius: 4,
                chunk_radius: 8,
                seed: 0,
            },
            Mode::Empty => simulation::Config {
                mode: *self,
                world_radius: 1,
                chunk_radius: 2,
                seed: 0,
            },
            Mode::WorldTest => simulation::Config {
                mode: *self,
                world_radius: 3,
                chunk_radius: 4,
                seed: 0,
            },
            Mode::GraphTest => simulation::Config {
                mode: *self,
                world_radius: 3,
                chunk_radius: 4,
                seed: 0,
            },
        }
    }
}
