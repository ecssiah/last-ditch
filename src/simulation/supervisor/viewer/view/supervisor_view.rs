use crate::simulation::supervisor::supervisor_status::SupervisorStatus;

#[derive(Clone)]
pub struct SupervisorView {
    pub supervisor_status: SupervisorStatus,
}

impl SupervisorView {
    pub fn new() -> Self {
        Self {
            supervisor_status: SupervisorStatus::Start,
        }
    }
}

impl Default for SupervisorView {
    fn default() -> Self {
        Self::new()
    }
}
