//! Simulation meta information

pub mod mode;

pub use mode::Mode;

use crate::simulation::consts::*;

#[derive(Debug)]
pub struct Admin {
    pub seed: u64,
    pub mode: Mode,
    pub message: String,
}

impl Admin {
    pub fn new() -> Self {
        Self {
            seed: DEFAULT_SEED,
            mode: Mode::Load,
            message: String::from("Loading World"),
        }
    }

    pub fn setup(&mut self) {
        self.mode = Mode::Simulate;
        self.message = format!("{} {}", PROJECT_TITLE, env!("CARGO_PKG_VERSION"));
    }
}
