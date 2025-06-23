//! Allows Interface to send messages to Simulation

use crate::simulation;
use tokio::sync::mpsc::UnboundedSender;

pub struct Dispatch {
    action_tx: UnboundedSender<simulation::state::receiver::action::Action>,
}

impl Dispatch {
    pub fn new(action_tx: UnboundedSender<simulation::state::receiver::action::Action>) -> Self {
        Self { action_tx }
    }

    pub fn send(&self, action: simulation::state::receiver::action::Action) {
        match self.action_tx.send(action) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Send failed: {:?}", e);
            }
        }
    }
}
