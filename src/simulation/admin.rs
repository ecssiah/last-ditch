//! The Simulation module contains all of the logic required to generate and evolve
//! the core civilizational garden.

pub mod mode;

pub use mode::Mode;

use crate::simulation::{consts::*, time::Tick};

#[derive(Debug)]
pub struct Admin {
    pub tick: Tick,
    pub seed: u64,
    pub mode: Mode,
    pub message: String,
    pub settlement_tick: u32,
}

impl Admin {
    pub fn new() -> Admin {
        let admin = Admin {
            tick: Tick::ZERO,
            seed: DEFAULT_SEED,
            mode: Mode::Load,
            message: String::from("Loading World"),
            settlement_tick: 0,
        };

        admin
    }
}
