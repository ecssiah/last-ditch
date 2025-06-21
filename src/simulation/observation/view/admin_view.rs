use crate::simulation::state::admin::Mode;

#[derive(Clone, Default, Debug)]
pub struct AdminView {
    pub mode: Mode,
    pub message: String,
}
