use crate::simulation::state::compute::task_output;
use std::collections::HashMap;

pub struct Store {
    pub path_region_data_map: HashMap<task_output::ID, task_output::data::path::Region>,
    pub path_local_data_map: HashMap<task_output::ID, task_output::data::path::Local>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            path_region_data_map: HashMap::new(),
            path_local_data_map: HashMap::new(),
        }
    }
}
