use crate::simulation::observation::state_pair::StatePair;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct TimeView {
    pub instant: StatePair<Instant>,
}
