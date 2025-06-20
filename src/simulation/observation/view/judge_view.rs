use crate::simulation::{
    observation::state_pair::StatePair,
    state::population::entity::{self, Kinematics, Spatial},
};

#[derive(Clone, Debug)]
pub struct JudgeView {
    pub id: entity::ID,
    pub spatial: StatePair<Spatial>,
    pub kinematics: StatePair<Kinematics>,
}

impl JudgeView {
    pub fn new() -> Self {
        Self {
            id: entity::ID::zero(),
            spatial: StatePair::new(Spatial::new(), Spatial::new()),
            kinematics: StatePair::new(Kinematics::new(), Kinematics::new()),
        }
    }
}
