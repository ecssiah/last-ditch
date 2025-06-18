use crate::simulation::state::admin::Mode;

#[derive(Clone, Debug)]
pub struct AdminView {
    pub mode: Mode,
    pub message: String,
}

impl AdminView {
    pub fn new() -> Self {
        Self {
            mode: Mode::Load,
            message: String::new(),
        }
    }
}
