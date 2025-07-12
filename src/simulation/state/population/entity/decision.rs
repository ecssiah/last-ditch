pub mod plan;

pub use plan::Plan;

use std::collections::{BinaryHeap, HashMap};

pub struct Decision {
    pub max_choices: u32,
    pub max_actions: u32,
    pub plan_map: HashMap<plan::ID, Plan>,
    pub active_plan_id: Option<plan::ID>,
    pub plan_heap: BinaryHeap<Plan>,
    pub plan_store: plan::Store,
}

impl Decision {
    pub fn new() -> Self {
        let max_choices = 4;
        let max_actions = 2;

        let plan_map = HashMap::new();
        let active_plan_id = None;

        let plan_heap = BinaryHeap::new();
        let plan_store = plan::Store::new();

        Self {
            max_choices,
            max_actions,
            plan_map,
            active_plan_id,
            plan_heap,
            plan_store,
        }
    }
}
