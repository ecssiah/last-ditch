use crate::simulation::{observation::state_pair::StatePair, population::judge, time::Tick};
use glam::{Quat, Vec3};

#[derive(Clone, Debug)]
pub struct JudgeView {
    pub tick: StatePair<Tick>,
    pub id: judge::ID,
    pub size: StatePair<Vec3>,
    pub position: StatePair<Vec3>,
    pub orientation: StatePair<Quat>,
}

impl JudgeView {
    pub fn new() -> JudgeView {
        let judge_view = JudgeView {
            tick: StatePair::new(Tick::ZERO, Tick::ZERO),
            id: judge::ID(0),
            position: StatePair::new(Vec3::ZERO, Vec3::ZERO),
            size: StatePair::new(Vec3::ZERO, Vec3::ZERO),
            orientation: StatePair::new(Quat::IDENTITY, Quat::IDENTITY),
        };

        judge_view
    }
}
