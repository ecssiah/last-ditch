use crate::simulation::state::compute::result;
use std::collections::HashMap;

pub struct Store {
    pub path_region_data_map: HashMap<result::ID, result::data::path::Region>,
    pub path_local_data_map: HashMap<result::ID, result::data::path::Local>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            path_region_data_map: HashMap::new(),
            path_local_data_map: HashMap::new(),
        }
    }
}
