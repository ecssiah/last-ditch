use crate::simulation::state::{
    population::{identity::Identity, kinematic::Kinematic, sight::Sight, spatial::Spatial},
    world::block,
};

#[derive(Clone, Debug)]
pub struct PersonView {
    pub identity: Identity,
    pub spatial: Spatial,
    pub kinematic: Kinematic,
    pub sight: Sight,
    pub selected_block_kind: block::Kind,
}

impl PersonView {
    pub fn new() -> Self {
        Self {
            identity: Identity::new(),
            spatial: Spatial::new(),
            kinematic: Kinematic::new(),
            sight: Sight::new(),
            selected_block_kind: block::Kind::EngravedStone1,
        }
    }
}
