use crate::simulation::state::world::grid::Line;
use ultraviolet::IVec3;

#[derive(Clone, Debug)]
pub struct Connection {
    pub area_id1: u64,
    pub area_id2: u64,
    pub entrance_vec: Vec<IVec3>,
    pub line: Line,
    pub cost: f32,
}
