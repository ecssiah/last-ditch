use crate::simulation;

#[derive(Clone, Copy)]
pub enum Mode {
    Main,
    WorldTest,
    GraphTest,
}

impl Mode {
    pub fn config(&self) -> simulation::Config {
        match self {
            Mode::Main => simulation::Config {
                mode: Mode::Main,
                world_radius: 4,
                chunk_radius: 8,
                seed: 0,
            },
            Mode::WorldTest => simulation::Config {
                mode: Mode::WorldTest,
                world_radius: 3,
                chunk_radius: 4,
                seed: 0,
            },
            Mode::GraphTest => simulation::Config {
                mode: Mode::GraphTest,
                world_radius: 3,
                chunk_radius: 4,
                seed: 0,
            },
        }
    }
}
