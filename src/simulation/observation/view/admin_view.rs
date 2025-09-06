use crate::simulation::state::admin::Mode;

#[derive(Clone, Debug)]
pub struct AdminView {
    pub mode: Mode,
    pub message: String,
    pub debug_active: bool,
}

impl AdminView {
    pub fn new() -> Self {
        Self {
            mode: Mode::Load,
            message: "NO MESSAGE SET".to_string(),
            debug_active: false,
        }
    }
}
