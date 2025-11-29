pub mod phase;
pub mod population_template;
pub mod world_template;

pub use phase::Phase;

use crate::simulation::state::{self, State};

pub struct Constructor {
    pub template: state::Template,
    pub phase: Phase,
}

impl Constructor {
    pub fn new(template: state::Template) -> Self {
        let phase = Phase::World;

        Self { template, phase }
    }

    pub fn tick(_state: &mut State, _constructor: &mut Constructor) {}
}
