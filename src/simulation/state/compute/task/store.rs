use crate::simulation::state::compute::task;
use std::collections::HashMap;

pub struct Store {
    pub path_region_data_map: HashMap<task::ID, task::data::path::Region>,
    pub path_local_data_map: HashMap<task::ID, task::data::path::Local>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            path_region_data_map: HashMap::new(),
            path_local_data_map: HashMap::new(),
        }
    }
}
