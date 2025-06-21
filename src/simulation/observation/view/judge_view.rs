use crate::simulation::{
    observation::state_pair::StatePair,
    state::population::entity::{self, Kinematic, Spatial},
};

#[derive(Clone, Debug)]
pub struct JudgeView {
    pub id: entity::ID,
    pub spatial: StatePair<Spatial>,
    pub kinematic: StatePair<Kinematic>,
}

impl JudgeView {
    pub fn new() -> Self {
        Self {
            id: entity::ID::default(),
            spatial: StatePair::new(Spatial::new(), Spatial::new()),
            kinematic: StatePair::new(Kinematic::new(), Kinematic::new()),
        }
    }
}
