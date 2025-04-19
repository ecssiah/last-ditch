use crate::simulation::observation::state_pair::StatePair;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct TimeView {
    pub instant: StatePair<Instant>,
}

impl TimeView {
    pub fn new() -> TimeView {
        let time_view = TimeView {
            instant: StatePair {
                current: Instant::now(),
                next: Instant::now(),
            },
        };

        time_view
    }
}
