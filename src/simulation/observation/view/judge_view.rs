use crate::simulation::{population::entity, time::Tick};
use glam::{Quat, Vec3};

#[derive(Clone, Debug)]
pub struct JudgeView {
    pub id: entity::ID,

    pub tick: (Tick, Tick),
    pub position: (Vec3, Vec3),
    pub orientation: (Quat, Quat),
}