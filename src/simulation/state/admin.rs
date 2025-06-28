//! Simulation meta information

pub mod mode;

pub use mode::Mode;

#[derive(Debug)]
pub struct Admin {
    pub mode: Mode,
    pub message: String,
}

impl Admin {
    pub fn new() -> Self {
        Self {
            mode: Mode::Menu,
            message: "NO MESSAGE SET".to_string(),
        }
    }

    pub fn setup(&mut self) {}

    pub fn tick(&mut self) {}
}

impl Default for Admin {
    fn default() -> Self {
        Self::new()
    }
}
