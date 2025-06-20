use crate::simulation::{
    observation::state_pair::StatePair,
    state::population::entity::{self, Kinematics, Spatial},
};

#[derive(Clone, Debug)]
pub struct AgentView {
    pub id: entity::ID,
    pub kind: entity::Kind,
    pub spatial: StatePair<Spatial>,
    pub kinematics: StatePair<Kinematics>,
}
