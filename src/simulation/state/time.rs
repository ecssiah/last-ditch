//! Time within simulation

pub mod tick;

pub use tick::Tick;
use tracing::info_span;

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
        let _time_span = info_span!("time_tick").entered();

        time.tick += 1;
    }
}
