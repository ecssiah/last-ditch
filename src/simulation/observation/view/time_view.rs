use crate::simulation::observation::state_pair::StatePair;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct TimeView {
    pub instant: StatePair<Instant>,
}

impl Default for TimeView {
    fn default() -> Self {
        Self {
            instant: StatePair::new(Instant::now(), Instant::now()),
        }
    }
}
