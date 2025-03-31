use crate::simulation::{
    id::{agent_id::AgentID, observation_id::ObservationID},
    observation::repository::Repository,
    state::State,
};

pub mod buffer;
pub mod repository;
pub mod view;

pub struct Observation {
    repository: repository::Repository,
}

impl Observation {
    pub fn new() -> Self {
        let repository = Repository::new();

        let observation = Self { repository };

        observation
    }

    pub fn update(&mut self, state: &State) {}

    pub fn request_view(&mut self, agent_id: AgentID) -> Option<ObservationID> {
        None
    }
}
