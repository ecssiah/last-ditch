//! Simulation meta information

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
    pub fn new() -> Self {
        Self {
            tick: Tick::ZERO,
            seed: DEFAULT_SEED,
            mode: Mode::Load,
            message: String::from("Loading World"),
            settlement_tick: 0,
        }
    }
}
