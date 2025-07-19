use crate::simulation::state::population::entity::{self};

#[derive(Clone, Debug)]
pub struct Nation {
    pub kind: entity::Kind,
}

impl Nation {
    pub fn new(kind: entity::Kind) -> Self {
        Self { kind }
    }
}
