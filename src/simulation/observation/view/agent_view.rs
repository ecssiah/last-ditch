use crate::simulation::state::population::entity::{self, Kinematic, Sense, Spatial, nation};

#[derive(Clone, Debug)]
pub struct AgentView {
    pub entity_kind: entity::Kind,
    pub nation_kind: nation::Kind,
    pub spatial: Spatial,
    pub kinematic: Kinematic,
    pub sense: Sense,
}

impl AgentView {
    pub fn new() -> Self {
        Self {
            entity_kind: entity::Kind::Agent,
            nation_kind: nation::Kind::Eagle,
            spatial: Spatial::new(),
            kinematic: Kinematic::new(),
            sense: Sense::new(),
        }
    }
}
