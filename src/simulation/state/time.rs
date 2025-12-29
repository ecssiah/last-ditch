//! Time within simulation

use std::time::Instant;

use tracing::instrument;

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

    #[instrument(skip_all, name = "tick")]
    pub fn tick(time: &mut Self) {
        time.tick += 1;
    }
}
