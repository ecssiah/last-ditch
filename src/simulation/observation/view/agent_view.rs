use crate::simulation::state::population::entity::{self, Detection, Kinematic, Spatial};

#[derive(Clone, Default, Debug)]
pub struct AgentView {
    pub id: entity::ID,
    pub kind: entity::Kind,
    pub spatial: Spatial,
    pub kinematic: Kinematic,
    pub detection: Detection,
}
