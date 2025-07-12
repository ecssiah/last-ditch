use crate::simulation::state::population::entity::decision::plan;
use glam::IVec3;

pub struct Travel {
    pub stage: plan::Stage,
    pub target_position: IVec3,
    pub path_found: bool,
    pub path_complete: bool,
    pub path_index: usize,
    pub local_path_found: bool,
    pub local_path_index: usize,
    pub region_path_vec: Vec<IVec3>,
    pub local_path_vec: Vec<IVec3>,
}

impl Travel {
    pub fn new(target_position: IVec3) -> Self {
        Self {
            stage: plan::Stage::Init,
            target_position,
            path_found: false,
            path_complete: false,
            path_index: 0,
            local_path_found: false,
            local_path_index: 0,
            region_path_vec: Vec::new(),
            local_path_vec: Vec::new(),
        }
    }
}
