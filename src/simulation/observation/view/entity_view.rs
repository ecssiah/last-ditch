use crate::simulation::{population::entity, time::Tick};
use glam::{Quat, Vec3};

#[derive(Clone)]
pub struct EntityView {
    pub id: entity::ID,

    pub tick: Tick,
    pub position: Vec3,
    pub orientation: Quat,

    pub next_tick: Tick,
    pub next_position: Vec3,
    pub next_orientation: Quat,
}
