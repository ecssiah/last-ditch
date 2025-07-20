use crate::simulation::state::compute::task;
use std::collections::HashMap;

pub struct Store {
    pub path_region_data_map: HashMap<task::output::ID, task::output::data::path::RegionData>,
    pub path_local_data_map: HashMap<task::output::ID, task::output::data::path::LocalData>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            path_region_data_map: HashMap::new(),
            path_local_data_map: HashMap::new(),
        }
    }
}
