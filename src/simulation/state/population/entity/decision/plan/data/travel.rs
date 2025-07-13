use crate::simulation::state::{
    population::entity::decision::plan,
    world::graph::{path, Path},
};
use glam::IVec3;

pub struct Travel {
    pub stage: plan::Stage,
    pub target_position: IVec3,
    pub region_path_found: bool,
    pub region_path_complete: bool,
    pub region_path_index: usize,
    pub region_path: Path,
    pub local_path_found: bool,
    pub local_path_index: usize,
    pub local_path: Path,
    pub next_local_path_found: bool,
    pub next_local_path_index: usize,
    pub next_local_path: Path,
}

impl Travel {
    pub fn new(target_position: IVec3) -> Self {
        Self {
            stage: plan::Stage::Init,
            target_position,
            region_path_found: false,
            region_path_complete: false,
            region_path_index: 0,
            region_path: Path::new(path::Kind::Region),
            local_path_found: false,
            local_path_index: 0,
            local_path: Path::new(path::Kind::Local),
            next_local_path_found: false,
            next_local_path_index: 0,
            next_local_path: Path::new(path::Kind::Local),
        }
    }
}
