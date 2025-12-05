use crate::simulation::manager::viewer::{PersonView, view::LeadershipView};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct PopulationView {
    pub leadership_view: LeadershipView,
    pub person_view_map: HashMap<u64, PersonView>,
}

impl PopulationView {
    pub fn new() -> Self {
        Self {
            leadership_view: LeadershipView::new(),
            person_view_map: HashMap::new(),
        }
    }
}
