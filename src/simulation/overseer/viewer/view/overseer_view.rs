use crate::simulation::overseer::overseer_status::OverseerStatus;

#[derive(Clone, Copy)]
pub struct OverseerView {
    pub overseer_status: OverseerStatus,
}

impl OverseerView {
    pub fn new() -> Self {
        Self {
            overseer_status: OverseerStatus::Run,
        }
    }

    pub fn default() -> Self {
        Self::new()
    }
}
