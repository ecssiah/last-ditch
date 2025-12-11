use ultraviolet::IVec3;
use crate::simulation::state::world::grid::Line;

#[derive(Clone, Debug)]
pub struct Connection {
    pub area_id1: u64,
    pub area_id2: u64,
    pub cost: f32,
    pub line: Line,
    pub grid_position: IVec3,
}
