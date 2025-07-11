use crate::simulation::state::{population::entity::decision::plan, time::Tick};

pub struct Idle {
    pub state: plan::State,
    pub tick_count: Tick,
    pub duration: Tick,
}

impl Idle {
    pub fn new(duration: Tick) -> Self {
        Self {
            state: plan::State::Init,
            tick_count: Tick::ZERO,
            duration,
        }
    }
}
