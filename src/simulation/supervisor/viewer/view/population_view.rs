use crate::simulation::{
    state::population::person::person_id::PersonID,
    supervisor::viewer::{view::LeadershipView, PersonView},
};
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct PopulationView {
    pub active: bool,
    pub leadership_view: LeadershipView,
    pub person_view_map: HashMap<PersonID, PersonView>,
}
