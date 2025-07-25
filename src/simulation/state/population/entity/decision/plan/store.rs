use crate::simulation::state::population::entity::decision::plan;
use std::collections::HashMap;

pub struct Store {
    pub travel_data_map: HashMap<plan::ID, plan::data::TravelData>,
    pub idle_data_map: HashMap<plan::ID, plan::data::IdleData>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            travel_data_map: HashMap::new(),
            idle_data_map: HashMap::new(),
        }
    }
}
