use crate::simulation::admin::Mode;

#[derive(Clone, Debug)]
pub struct AdminView {
    pub mode: Mode,
}

impl AdminView {
    pub fn new() -> AdminView {
        let admin_view = AdminView { mode: Mode::Load };

        admin_view
    }
}
