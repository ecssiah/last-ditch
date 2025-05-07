use crate::simulation::{observation::state_pair::StatePair, population::judge, time::Tick};
use glam::{Quat, Vec3};

#[derive(Clone, Debug)]
pub struct JudgeView {
    pub tick: StatePair<Tick>,
    pub id: judge::ID,
    pub position: StatePair<Vec3>,
    pub orientation: StatePair<Quat>,
}

impl JudgeView {
    pub fn new() -> JudgeView {
        let judge_view = JudgeView {
            tick: StatePair::new(Tick::ZERO, Tick::ZERO),
            id: judge::ID(0),
            position: StatePair {
                current: Vec3::ZERO,
                next: Vec3::ZERO,
            },
            orientation: StatePair {
                current: Quat::IDENTITY,
                next: Quat::IDENTITY,
            },
        };

        judge_view
    }
}
