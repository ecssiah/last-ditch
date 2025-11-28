use crate::simulation::manager::status::Status;

#[derive(Clone, Copy)]
pub struct ManagerView {
    pub status: Status,
}

impl ManagerView {
    pub fn new() -> Self {
        Self {
            status: Status::Init,
        }
    }
}