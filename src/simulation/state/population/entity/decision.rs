use crate::simulation::state::population::entity::decision::plan::Plan;
use std::collections::BinaryHeap;

pub mod plan;

pub struct Decision {
    pub max_choices: u32,
    pub max_actions: u32,
    pub plan_heap: BinaryHeap<Plan>,
    pub plan_data: plan::Data,
}

impl Decision {
    pub fn new() -> Self {
        let max_choices = 4;
        let max_actions = 2;

        let plan_heap = BinaryHeap::new();
        let plan_data = plan::Data::new();

        Self {
            max_choices,
            max_actions,
            plan_heap,
            plan_data,
        }
    }
}
