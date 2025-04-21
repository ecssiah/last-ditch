use crate::simulation::{observation::state_pair::StatePair, time::Tick};
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct TimeView {
    pub tick: StatePair<Tick>,
    pub instant: StatePair<Instant>,
}

impl TimeView {
    pub fn new() -> TimeView {
        let time_view = TimeView {
            tick: StatePair {
                current: Tick::ZERO,
                next: Tick::ZERO,
            },
            instant: StatePair {
                current: Instant::now(),
                next: Instant::now(),
            },
        };

        time_view
    }
}
