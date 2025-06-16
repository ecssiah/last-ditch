use crate::simulation::{
    observation::state_pair::StatePair, physics::aabb::AABB, population::judge,
};
use glam::{Quat, Vec3};

#[derive(Clone, Debug)]
pub struct JudgeView {
    pub id: judge::ID,
    pub world_position: StatePair<Vec3>,
    pub aabb: StatePair<AABB>,
    pub orientation: StatePair<Quat>,
}

impl JudgeView {
    pub fn new() -> Self {
        Self {
            id: judge::ID(0),
            world_position: StatePair::default(),
            aabb: StatePair::default(),
            orientation: StatePair::default(),
        }
    }
}
