pub mod mode;

pub use mode::Mode;

use crate::simulation::consts::*;

pub struct Admin {
    pub seed: u64,
    pub mode: Mode,
    pub message: String,
    pub settlement_tick: u32,
}

impl Admin {
    pub fn new() -> Admin {
        let admin = Admin {
            seed: DEFAULT_SEED,
            mode: Mode::Load,
            message: String::new(),
            settlement_tick: 0,
        };

        admin
    }

    pub fn is_settling(&self) -> bool {
        self.settlement_tick < SIMULATION_SETTLEMENT_PERIOD
    }
}
