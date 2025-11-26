use crate::simulation::state::population::{self, kinematic::Kinematic, nation, spatial::Spatial};

#[derive(Clone, Debug)]
pub struct AgentView {
    pub role: population::Role,
    pub nation_kind: nation::Kind,
    pub spatial: Spatial,
    pub kinematic: Kinematic,
}

impl AgentView {
    pub fn new() -> Self {
        Self {
            role: population::Role::Agent,
            nation_kind: nation::Kind::Eagle,
            spatial: Spatial::new(),
            kinematic: Kinematic::new(),
        }
    }
}
