//! Allows Interface to send messages to Simulation

use crate::simulation::state::receiver::action::Action;
use tokio::sync::mpsc::{error::SendError, UnboundedSender};

pub struct Dispatch {
    action_tx: UnboundedSender<Action>,
}

impl Dispatch {
    pub fn new(action_tx: UnboundedSender<Action>) -> Self {
        Self { action_tx }
    }

    pub fn send(&self, action: Action) -> Result<(), SendError<Action>> {
        self.action_tx.send(action)
    }
}
