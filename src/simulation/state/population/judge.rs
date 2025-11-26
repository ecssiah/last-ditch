pub mod id;

pub use id::ID;

use crate::simulation::state::{
    population::{
        self, identity::Identity, kinematic::Kinematic, nation, sight::Sight, spatial::Spatial,
    },
    World,
};

pub struct Judge {
    pub id: ID,
    pub identity: Identity,
    pub spatial: Spatial,
    pub kinematic: Kinematic,
    pub sight: Sight,
}

impl Judge {
    pub fn new() -> Self {
        let id = ID::allocate();

        let identity = Identity {
            role: population::Role::Judge,
            nation_kind: nation::Kind::Eagle,
        };

        let spatial = Spatial::new();
        let kinematic = Kinematic::new();
        let sight = Sight::new();

        Self {
            id,
            identity,
            spatial,
            kinematic,
            sight,
        }
    }

    pub fn tick(world: &World, judge: &mut Judge) {
        Spatial::update_sector_id(&world.grid, &mut judge.spatial);
    }
}
