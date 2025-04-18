use std::time::Instant;

#[derive(Clone, Debug)]
pub struct TimeView {
    pub simulation_instant: (Instant, Instant),
}
