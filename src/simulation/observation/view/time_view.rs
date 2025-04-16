use std::time::Instant;

#[derive(Clone, Debug)]
pub struct TimeView {
    pub simulation_instant: Instant,
    
    pub next_simulation_instant: Instant,
}
