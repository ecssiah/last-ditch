use crate::simulation::state::{
    navigation::Navigation,
    population::{self, identity::Identity, kinematic::Kinematic, nation, spatial::Spatial},
};

pub struct Agent {
    pub entity_id: u64,
    pub identity: Identity,
    pub spatial: Spatial,
    pub kinematic: Kinematic,
}

impl Agent {
    pub fn new(entity_id: u64, nation_kind: nation::Kind) -> Self {
        let identity = Identity {
            role: population::Role::Agent,
            nation_kind,
        };

        let spatial = Spatial::new();
        let kinematic = Kinematic::new();

        Self {
            entity_id,
            identity,
            spatial,
            kinematic,
        }
    }

    pub fn tick(_navigation: &mut Navigation, agent: &mut Agent) {
        Spatial::update_sector_id(&mut agent.spatial);
    }
}
