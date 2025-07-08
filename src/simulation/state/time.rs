//! Time within simulation

pub mod tick;

pub use tick::Tick;

use std::time::Instant;

pub struct Time {
    pub tick: Tick,
    pub instant: Instant,
}

impl Time {
    pub fn new() -> Self {
        Self {
            tick: Tick::ZERO,
            instant: Instant::now(),
        }
    }

    pub fn tick(time: &mut Time) {
        time.tick += 1;
    }
}
