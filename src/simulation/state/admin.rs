//! Simulation meta information

pub mod mode;

pub use mode::Mode;

use crate::simulation::{consts::*, state::receiver::action::AdminAction};

#[derive(Debug)]
pub struct Admin {
    pub mode: Mode,
    pub message: String,
}

impl Admin {
    pub fn new() -> Self {
        Self {
            mode: Mode::Load,
            message: String::from("Loading World"),
        }
    }

    pub fn setup(&mut self) {
        self.mode = Mode::Simulate;
        self.message = format!("{} {}", PROJECT_TITLE, env!("CARGO_PKG_VERSION"));
    }

    pub fn receive_action(&mut self, admin_action: &AdminAction) {
        match admin_action {
            AdminAction::Exit => self.receive_exit_action(),
        }
    }

    pub fn receive_exit_action(&mut self) {
        self.mode = Mode::Exit;
    }
}

impl Default for Admin {
    fn default() -> Self {
        Self::new()
    }
}
