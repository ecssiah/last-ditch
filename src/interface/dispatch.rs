//! Sends Actions from Interface to Simulation

use crate::simulation::state;
use tokio::sync::mpsc::UnboundedSender;

pub struct Dispatch {
    pub action_tx: UnboundedSender<state::Action>,
}

impl Dispatch {
    pub fn new(action_tx: UnboundedSender<state::Action>) -> Self {
        Self { action_tx }
    }
}
