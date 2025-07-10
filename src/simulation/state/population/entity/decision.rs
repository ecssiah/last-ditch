use crate::simulation::state::{
    population::entity::decision::{coordinator::Coordinator, plan::Plan},
    World,
};
use std::collections::HashMap;

pub mod coordinator;
pub mod plan;

pub struct Decision {
    pub max_choices: u32,
    pub max_actions: u32,
    pub coordinator: Coordinator,
    pub plan_map: HashMap<plan::Priority, Vec<Plan>>,
}

impl Decision {
    pub fn new() -> Self {
        let max_choices = 4;
        let max_actions = 2;

        let coordinator = Coordinator::new();

        let plan_map = HashMap::from([
            (plan::Priority::High, Vec::new()),
            (plan::Priority::Medium, Vec::new()),
            (plan::Priority::Low, Vec::new()),
        ]);

        Self {
            max_choices,
            max_actions,
            coordinator,
            plan_map,
        }
    }

    pub fn tick(decision: &mut Decision, world: &World) {
        let mut choices = 0;

        loop {
            for plan in &decision.plan_map[&plan::Priority::High] {}
        }
    }
}
