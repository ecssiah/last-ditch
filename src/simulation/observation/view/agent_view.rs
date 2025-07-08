use crate::simulation::state::population::entity::{self, Detection, Kinematic, Spatial};

#[derive(Clone, Debug)]
pub struct AgentView {
    pub id: entity::ID,
    pub kind: entity::Kind,
    pub spatial: Spatial,
    pub kinematic: Kinematic,
    pub detection: Detection,
}

impl AgentView {
    pub fn new() -> Self {
        Self {
            id: entity::ID::MAX,
            kind: entity::Kind::Eagle,
            spatial: Spatial::new(),
            kinematic: Kinematic::new(),
            detection: Detection::new(),
        }
    }
}
