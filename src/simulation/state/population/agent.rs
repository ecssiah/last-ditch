use crate::simulation::state::{
    population::{self, identity::Identity, kinematic::Kinematic, nation, spatial::Spatial},
    World,
};

pub struct Agent {
    pub agent_id: u64,
    pub identity: Identity,
    pub spatial: Spatial,
    pub kinematic: Kinematic,
}

impl Agent {
    pub fn new(agent_id: u64, nation_kind: nation::Kind) -> Self {
        let identity = Identity {
            role: population::Role::Agent,
            nation_kind,
        };

        let spatial = Spatial::new();
        let kinematic = Kinematic::new();

        Self {
            agent_id,
            identity,
            spatial,
            kinematic,
        }
    }

    pub fn tick(_world: &World, agent: &mut Agent) {
        Spatial::update_sector_id(&mut agent.spatial);
    }
}
