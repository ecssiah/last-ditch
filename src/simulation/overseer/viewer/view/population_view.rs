use crate::simulation::overseer::viewer::{view::LeadershipView, PersonView};
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct PopulationView {
    pub active: bool,
    pub leadership_view: LeadershipView,
    pub person_view_map: HashMap<u64, PersonView>,
}
