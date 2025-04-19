use crate::simulation::{observation::state_pair::StatePair, population::judge, time::Tick};
use glam::{Quat, Vec3};

#[derive(Clone, Debug)]
pub struct JudgeView {
    pub id: judge::ID,

    pub tick: StatePair<Tick>,
    pub position: StatePair<Vec3>,
    pub orientation: StatePair<Quat>,
}
