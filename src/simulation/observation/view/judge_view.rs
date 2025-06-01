use crate::simulation::{
    observation::state_pair::StatePair, physics::aabb::AABB, population::judge, time::Tick,
};
use glam::{Quat, Vec3};

#[derive(Clone, Debug)]
pub struct JudgeView {
    pub id: judge::ID,
    pub tick: StatePair<Tick>,
    pub position: StatePair<Vec3>,
    pub aabb: StatePair<AABB>,
    pub orientation: StatePair<Quat>,
}

impl JudgeView {
    pub fn new() -> JudgeView {
        let judge_view = JudgeView {
            id: judge::ID(0),
            tick: StatePair::default(),
            position: StatePair::default(),
            aabb: StatePair::default(),
            orientation: StatePair::default(),
        };

        judge_view
    }
}
