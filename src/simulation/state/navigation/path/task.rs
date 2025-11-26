use crate::simulation::state::navigation::path;

pub struct Task {
    pub path_request: path::Request,
    pub path_state: path::State,
    pub finished: bool,
}
