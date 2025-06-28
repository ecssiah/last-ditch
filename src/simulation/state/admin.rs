//! Simulation meta information

pub mod mode;

pub use mode::Mode;

use crate::simulation::{consts::*, state::receiver::action::AdminAction};

#[derive(Debug)]
pub struct Admin {
    pub action_vec: Vec<AdminAction>,
    pub mode: Mode,
    pub message: String,
}

impl Admin {
    pub fn new() -> Self {
        Self {
            action_vec: Vec::default(),
            mode: Mode::Load,
            message: String::from("Loading World"),
        }
    }

    pub fn setup(&mut self) {
        self.mode = Mode::Simulate;
        self.message = format!("{} {}", PROJECT_TITLE, env!("CARGO_PKG_VERSION"));
    }

    pub fn tick(&mut self) {
        let action_vec = std::mem::take(&mut self.action_vec);

        for action in action_vec {
            match action {
                AdminAction::Exit => self.apply_exit_action(),
            }
        }
    }

    pub fn receive_action(&mut self, admin_action: &AdminAction) {
        self.action_vec.push(*admin_action);
    }

    pub fn apply_exit_action(&mut self) {
        self.mode = Mode::Exit;
    }
}

impl Default for Admin {
    fn default() -> Self {
        Self::new()
    }
}
