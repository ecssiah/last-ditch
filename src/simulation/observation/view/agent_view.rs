use crate::simulation::{
    observation::state_pair::StatePair,
    state::population::entity::{self, Kinematic, Spatial},
};

#[derive(Clone, Default, Debug)]
pub struct AgentView {
    pub id: entity::ID,
    pub kind: entity::Kind,
    pub spatial: StatePair<Spatial>,
    pub kinematic: StatePair<Kinematic>,
}
