use crate::simulation::state::population::entity::decision::plan;
use glam::IVec3;

pub struct Travel {
    pub state: plan::State,
    pub region_path_found: bool,
    pub region_path_tracking: bool,
    pub region_path_complete: bool,
    pub region_path_vec: Vec<IVec3>,
    pub local_path_found: bool,
    pub local_path_tracking: bool,
    pub local_path_complete: bool,
    pub local_path_vec: Vec<IVec3>,
}

impl Travel {
    pub fn new() -> Self {
        Self {
            state: plan::State::Init,
            region_path_found: false,
            region_path_tracking: false,
            region_path_complete: false,
            region_path_vec: Vec::new(),
            local_path_found: false,
            local_path_tracking: false,
            local_path_complete: false,
            local_path_vec: Vec::new(),
        }
    }
}
