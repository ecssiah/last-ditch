use crate::simulation::overseer::overseer_status::OverseerStatus;

#[derive(Clone)]
pub struct OverseerView {
    pub overseer_status: OverseerStatus,
}

impl OverseerView {
    pub fn new() -> Self {
        Self {
            overseer_status: OverseerStatus::Start,
        }
    }
}

impl Default for OverseerView {
    fn default() -> Self {
        Self::new()
    }
}
