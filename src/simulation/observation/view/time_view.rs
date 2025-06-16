use crate::simulation::observation::state_pair::StatePair;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct TimeView {
    pub instant: StatePair<Instant>,
}

impl TimeView {
    pub fn new() -> Self {
        Self {
            instant: StatePair {
                current: Instant::now(),
                next: Instant::now(),
            },
        }
    }
}
