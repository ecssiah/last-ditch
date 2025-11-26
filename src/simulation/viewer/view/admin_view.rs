use crate::simulation::state::admin;

#[derive(Clone, Debug)]
pub struct AdminView {
    pub mode: admin::Mode,
    pub message: String,
    pub debug_active: bool,
}

impl AdminView {
    pub fn new() -> Self {
        Self {
            mode: admin::Mode::Loading,
            message: "NO MESSAGE SET".to_string(),
            debug_active: false,
        }
    }
}
