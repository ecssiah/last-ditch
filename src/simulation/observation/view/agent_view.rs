use crate::simulation::state::population::{
    entity::{self, Detection, Kinematic, Spatial},
    nation,
};

#[derive(Clone, Debug)]
pub struct AgentView {
    pub id: entity::ID,
    pub entity_kind: entity::Kind,
    pub nation_kind: nation::Kind,
    pub spatial: Spatial,
    pub kinematic: Kinematic,
    pub detection: Detection,
}

impl AgentView {
    pub fn new() -> Self {
        Self {
            id: entity::ID::MAX,
            entity_kind: entity::Kind::Agent,
            nation_kind: nation::Kind::Eagle,
            spatial: Spatial::new(),
            kinematic: Kinematic::new(),
            detection: Detection::new(),
        }
    }
}
