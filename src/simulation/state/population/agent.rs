use crate::simulation::state::{
    navigation::Navigation,
    population::{
        self, behavior::Behavior, identity::Identity, kinematic::Kinematic, nation,
        spatial::Spatial,
    },
};

pub struct Agent {
    pub entity_id: u64,
    pub identity: Identity,
    pub spatial: Spatial,
    pub kinematic: Kinematic,
    pub behavior: Behavior,
}

impl Agent {
    pub fn new(entity_id: u64, nation_kind: nation::Kind) -> Self {
        let identity = Identity {
            role: population::Role::Agent,
            nation_kind,
        };

        let spatial = Spatial::new();
        let kinematic = Kinematic::new();
        let behavior = Behavior::new();

        Self {
            entity_id,
            identity,
            spatial,
            kinematic,
            behavior,
        }
    }

    pub fn tick(navigation: &mut Navigation, agent: &mut Agent) {
        Behavior::tick(navigation, agent);

        Spatial::update_sector_id(&mut agent.spatial);
    }
}
