use crate::simulation::{
    manager::viewer::{AgentView, JudgeView}, state::population::agent
};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct PopulationView {
    pub judge_view: JudgeView,
    pub agent_view_map: HashMap<agent::ID, AgentView>,
}

impl PopulationView {
    pub fn new() -> Self {
        Self {
            judge_view: JudgeView::new(),
            agent_view_map: HashMap::new(),
        }
    }
}
