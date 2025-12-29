use crate::simulation::state::{
    physics::body::Body,
    population::{identity::Identity, motion::Motion, sight::Sight, transform::Transform}, world::block, 
};

#[derive(Clone, Debug)]
pub struct PersonView {
    pub identity: Identity,
    pub transform: Transform,
    pub motion: Motion,
    pub body: Body,
    pub sight: Sight,
    pub selected_block_kind: block::Kind,
}

impl PersonView {
    pub fn new() -> Self {
        Self {
            identity: Identity::default(),
            transform: Transform::default(),
            motion: Motion::default(),
            sight: Sight::default(),
            body: Body::default(),
            selected_block_kind: block::Kind::Engraved1,
        }
    }

    pub fn default() -> Self {
        Self::new()
    }
}
