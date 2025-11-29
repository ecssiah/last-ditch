pub mod id;

pub use id::ID;

use crate::simulation::state::{
    population::{self, agent, identity::Identity, kinematic::Kinematic, nation, spatial::Spatial},
    World,
};

pub struct Agent {
    pub id: agent::ID,
    pub identity: Identity,
    pub spatial: Spatial,
    pub kinematic: Kinematic,
}

impl Agent {
    pub fn new(nation_kind: nation::Kind) -> Self {
        let id = ID::allocate();

        let identity = Identity {
            role: population::Role::Agent,
            nation_kind,
        };

        let spatial = Spatial::new();
        let kinematic = Kinematic::new();

        Self {
            id,
            identity,
            spatial,
            kinematic,
        }
    }

    pub fn tick(_world: &World, agent: &mut Agent) {
        Spatial::update_sector_id(&mut agent.spatial);
    }
}
