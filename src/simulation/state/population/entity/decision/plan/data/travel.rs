use crate::simulation::state::{population::entity::decision::plan, world::graph::Path};
use glam::IVec3;

#[derive(Debug)]
pub struct Travel {
    pub stage: plan::Stage,
    pub target_position: IVec3,
    pub path: Option<Path>,
}

impl Travel {
    pub fn new(target_position: IVec3) -> Self {
        Self {
            stage: plan::Stage::Init,
            target_position,
            path: None,
        }
    }
}
