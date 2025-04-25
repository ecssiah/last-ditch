use crate::simulation::admin::Mode;

#[derive(Clone, Debug)]
pub struct AdminView {
    pub mode: Mode,
    pub message: String,
}

impl AdminView {
    pub fn new() -> AdminView {
        let admin_view = AdminView {
            mode: Mode::Load,
            message: String::new(),
        };

        admin_view
    }
}
