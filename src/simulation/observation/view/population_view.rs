use crate::simulation::{
    observation::view::{AgentView, JudgeView},
    state::population::entity,
};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct PopulationView {
    pub judge_view: JudgeView,
    pub agent_view_map: HashMap<entity::ID, AgentView>,
}

impl PopulationView {
    pub fn new() -> Self {
        Self {
            judge_view: JudgeView::new(),
            agent_view_map: HashMap::new(),
        }
    }
}
