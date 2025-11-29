pub mod phase;
pub mod population_template;
pub mod world_template;

pub use phase::Phase;

use crate::simulation::state::State;

pub struct Constructor {
    pub phase: Phase,
}

impl Constructor {
    pub fn new() -> Self {
        let phase = Phase::World;

        Self { phase }
    }

    pub fn tick(state: &mut State, constructor: &mut Constructor) {}
}
