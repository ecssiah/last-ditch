//! Sends Actions from Interface to Simulation

use crate::simulation::state::receiver::action::Action;
use tokio::sync::mpsc::UnboundedSender;

pub struct Dispatch {
    pub action_tx: UnboundedSender<Action>,
}

impl Dispatch {
    pub fn new(action_tx: UnboundedSender<Action>) -> Self {
        Self { action_tx }
    }
}
