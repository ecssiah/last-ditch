pub mod plan;

pub use plan::Plan;

use std::collections::BinaryHeap;

use crate::simulation::state::time::Tick;

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

        let mut plan_heap = BinaryHeap::new();
        let mut plan_data = plan::Data::new();

        let idle_plan = Plan::new(plan::Priority::High, plan::Kind::Idle);
        let idle_data = plan::data::Idle::new(Tick::new(20));

        plan_data.idle_data.insert(idle_plan.id, idle_data);
        plan_heap.push(idle_plan);

        Self {
            max_choices,
            max_actions,
            plan_heap,
            plan_data,
        }
    }
}
