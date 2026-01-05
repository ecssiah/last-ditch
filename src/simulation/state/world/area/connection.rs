use crate::simulation::state::world::{area::area_id::AreaID, grid::Line};
use ultraviolet::IVec3;

#[derive(Clone, Debug)]
pub struct Connection {
    pub area_id1: AreaID,
    pub area_id2: AreaID,
    pub entrance_vec: Vec<IVec3>,
    pub line: Line,
    pub cost: f32,
}
