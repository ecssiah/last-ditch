use crate::simulation::state::{population::entity::decision::plan, time::Tick};

pub struct Idle {
    pub stage: plan::Stage,
    pub tick_count: Tick,
    pub tick_duration: Tick,
}

impl Idle {
    pub fn new(tick_duration: u32) -> Self {
        Self {
            stage: plan::Stage::Init,
            tick_count: Tick::ZERO,
            tick_duration: Tick::new(tick_duration),
        }
    }
}
