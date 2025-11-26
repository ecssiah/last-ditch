//! Sends Actions from Interface to Simulation

use crate::simulation::state::action::Act;
use tokio::sync::mpsc::UnboundedSender;

pub struct Dispatch {
    pub act_tx: UnboundedSender<Act>,
}

impl Dispatch {
    pub fn new(act_tx: UnboundedSender<Act>) -> Self {
        Self { act_tx }
    }
}
