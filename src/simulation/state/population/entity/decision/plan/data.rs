pub mod idle;
pub mod travel;

pub use idle::Idle;
pub use travel::Travel;

use crate::simulation::state::population::entity::decision::plan;
use std::collections::HashMap;

pub struct Data {
    pub idle_data: HashMap<plan::ID, plan::data::Idle>,
    pub travel_data: HashMap<plan::ID, plan::data::Travel>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            idle_data: HashMap::new(),
            travel_data: HashMap::new(),
        }
    }
}
