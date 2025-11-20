pub mod id;

pub use id::ID;

use crate::simulation::state::{
    population::entity::{self, nation, Entity, Info, Kinematic, Sense, Spatial},
    world::{sector, World},
};

pub struct Agent {
    pub agent_id: ID,
    pub entity: Entity,
}

impl Agent {
    pub fn new(nation_kind: nation::Kind) -> Self {
        let agent_id = ID::allocate();

        let info = Info {
            entity_kind: entity::Kind::Agent,
            sector_id: sector::ID(0),
            sector_updated: false,
            nation_kind,
        };

        let spatial = Spatial::new();
        let kinematic = Kinematic::new();
        let sense = Sense::new();

        let entity = Entity {
            info,
            spatial,
            kinematic,
            sense,
        };

        Self { agent_id, entity }
    }

    pub fn tick(world: &World, agent: &mut Agent) {
        Entity::tick(world, &mut agent.entity);
    }
}
