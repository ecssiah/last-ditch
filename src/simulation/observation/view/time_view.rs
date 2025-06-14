use crate::simulation::{observation::state_pair::StatePair, time::Tick};
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct TimeView {
    pub tick: StatePair<Tick>,
    pub instant: StatePair<Instant>,
}

impl TimeView {
    pub fn new() -> Self {
        Self {
            tick: StatePair::new(Tick::ZERO, Tick::ZERO),
            instant: StatePair {
                current: Instant::now(),
                next: Instant::now(),
            },
        }
    }
}
