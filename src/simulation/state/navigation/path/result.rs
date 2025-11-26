use crate::simulation::state::navigation::path;
use ultraviolet::IVec3;

pub struct Result {
    pub path_id: path::ID,
    pub path_vec: Vec<IVec3>,
}
