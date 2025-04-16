use std::time::Instant;

#[derive(Clone)]
pub struct TimeView {
    pub simulation_instant: Instant,
    
    pub next_simulation_instant: Instant,
}
