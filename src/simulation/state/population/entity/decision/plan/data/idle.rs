use crate::simulation::state::time::Tick;

pub struct Idle {
    pub tick_count: Tick,
    pub duration: Tick,
}

impl Idle {
    pub fn new(duration: Tick) -> Self {
        Self {
            tick_count: Tick::ZERO,
            duration,
        }
    }
}
