//! Simulation meta information

pub mod mode;

pub use mode::Mode;

#[derive(Debug)]
pub struct Admin {
    pub mode: Mode,
    pub message: String,
    pub debug_active: bool,
}

impl Admin {
    pub fn new() -> Self {
        Self {
            mode: Mode::Menu,
            message: "NO MESSAGE SET".to_string(),
            debug_active: false,
        }
    }

    pub fn toggle_debug(admin: &mut Admin) {
        admin.debug_active = !admin.debug_active
    }
}
