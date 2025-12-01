//! Time within simulation

use std::time::Instant;

pub struct Time {
    pub tick: u64,
    pub instant: Instant,
}

impl Time {
    pub fn new() -> Self {
        Self {
            tick: 0,
            instant: Instant::now(),
        }
    }

    pub fn tick(time: &mut Time) {
        let _ = tracing::info_span!("time_tick").entered();

        time.tick += 1;
    }
}
