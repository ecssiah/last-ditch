use crate::simulation::state::navigation::path;
use ultraviolet::IVec3;

pub struct Request {
    pub path_id: path::ID,
    pub start: IVec3,
    pub end: IVec3,
}
