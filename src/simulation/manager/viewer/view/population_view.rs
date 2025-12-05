use crate::simulation::manager::viewer::PersonView;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct PopulationView {
    pub judge_id: u64,
    pub person_view_map: HashMap<u64, PersonView>,
}

impl PopulationView {
    pub fn new() -> Self {
        Self {
            judge_id: 0,
            person_view_map: HashMap::new(),
        }
    }
}
