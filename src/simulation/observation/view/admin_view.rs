use crate::simulation::{admin::Mode, observation::state_pair::StatePair, time::Tick};

#[derive(Clone, Debug)]
pub struct AdminView {
    pub tick: StatePair<Tick>,
    pub mode: Mode,
    pub message: String,
}

impl AdminView {
    pub fn new() -> Self {
        Self {
            tick: StatePair::new(Tick::ZERO, Tick::ZERO),
            mode: Mode::Load,
            message: String::new(),
        }
    }
}
