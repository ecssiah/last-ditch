//! Time within simulation

pub mod tick;

pub use tick::Tick;

use std::time::Instant;

pub struct Time {
    pub tick: Tick,
    pub instant: Instant,
}

impl Time {
    pub fn new() -> Time {
        let time = Time {
            tick: Tick::ZERO,
            instant: Instant::now(),
        };

        time
    }

    pub fn tick(&mut self) {
        self.tick += 1;
    }
}
