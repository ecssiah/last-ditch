use crate::simulation::state::{
    population::{identity::Identity, kinematic::Kinematic, sight::Sight, transform::Transform},
    world::block,
};

#[derive(Clone, Debug)]
pub struct PersonView {
    pub identity: Identity,
    pub transform: Transform,
    pub kinematic: Kinematic,
    pub sight: Sight,
    pub selected_block_kind: block::Kind,
}

impl PersonView {
    pub fn new() -> Self {
        Self {
            identity: Identity::default(),
            transform: Transform::default(),
            kinematic: Kinematic::default(),
            sight: Sight::default(),
            selected_block_kind: block::Kind::Engraved1,
        }
    }

    pub fn default() -> Self {
        Self::new()
    }
}
