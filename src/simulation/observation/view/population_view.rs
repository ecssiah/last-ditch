use crate::simulation::{
    observation::{
        state_pair::StatePair,
        view::{AgentView, JudgeView},
    },
    population::agent,
    time::Tick,
};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct PopulationView {
    pub tick: StatePair<Tick>,
    pub judge_view: JudgeView,
    pub agent_view_map: HashMap<agent::ID, AgentView>,
}

impl PopulationView {
    pub fn new() -> Self {
        Self {
            tick: StatePair::new(Tick::ZERO, Tick::ZERO),
            judge_view: JudgeView::new(),
            agent_view_map: HashMap::new(),
        }
    }
}
