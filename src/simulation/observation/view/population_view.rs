use crate::simulation::{
    observation::{
        state_pair::StatePair,
        view::{AgentView, JudgeView},
    },
    population::entity,
    time::Tick,
};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct PopulationView {
    pub tick: StatePair<Tick>,
    pub judge_view: Option<JudgeView>,
    pub agent_views: HashMap<entity::ID, AgentView>,
}
