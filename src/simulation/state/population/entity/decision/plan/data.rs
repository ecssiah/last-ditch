use crate::simulation::state::{
    population::entity::decision::plan, time::Tick, world::graph::Path,
};
use glam::IVec3;

pub struct IdleData {
    pub stage: plan::Stage,
    pub tick_count: Tick,
    pub tick_duration: Tick,
}

impl IdleData {
    pub fn new(tick_duration: u32) -> Self {
        Self {
            stage: plan::Stage::Init,
            tick_count: Tick::ZERO,
            tick_duration: Tick::new(tick_duration),
        }
    }
}

#[derive(Debug)]
pub struct TravelData {
    pub stage: plan::Stage,
    pub target_position: IVec3,
    pub path: Option<Path>,
}

impl TravelData {
    pub fn new(target_position: IVec3) -> Self {
        Self {
            stage: plan::Stage::Init,
            target_position,
            path: None,
        }
    }
}
