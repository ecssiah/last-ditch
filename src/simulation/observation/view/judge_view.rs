use crate::simulation::{
    observation::state_pair::StatePair,
    state::population::entity::{self, Detection, Kinematic, Spatial},
};

#[derive(Clone, Default, Debug)]
pub struct JudgeView {
    pub id: entity::ID,
    pub spatial: StatePair<Spatial>,
    pub kinematic: StatePair<Kinematic>,
    pub detection: StatePair<Detection>,
}
