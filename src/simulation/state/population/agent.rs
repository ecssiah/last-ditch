use crate::simulation::state::{
    navigation::Navigation,
    population::{
        self, behavior::Behavior, identity::Identity, kinematic::Kinematic, nation,
        spatial::Spatial,
    },
};

pub struct Agent {
    pub agent_id: u64,
    pub identity: Identity,
    pub spatial: Spatial,
    pub kinematic: Kinematic,
    pub behavior: Behavior,
}

impl Agent {
    pub fn new(agent_id: u64, nation_kind: nation::Kind) -> Self {
        let identity = Identity {
            role: population::Role::Agent,
            nation_kind,
        };

        let spatial = Spatial::new();
        let kinematic = Kinematic::new();
        let behavior = Behavior::new();

        Self {
            agent_id,
            identity,
            spatial,
            kinematic,
            behavior,
        }
    }

    pub fn tick(navigation: &mut Navigation, agent: &mut Agent) {
        Spatial::update_sector_id(&mut agent.spatial);
        Behavior::tick(navigation, agent);
    }
}
