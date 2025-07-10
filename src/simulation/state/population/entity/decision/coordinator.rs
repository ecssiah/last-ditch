use crate::simulation::state::population::entity::decision::plan::{self, Plan};
use std::collections::HashMap;

pub struct Coordinator {}

impl Coordinator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn tick(coordinator: &mut Coordinator, plan_map: &mut HashMap<plan::Priority, Vec<Plan>>) {}
}
