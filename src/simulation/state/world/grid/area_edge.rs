use crate::simulation::state::world::grid::Line;

pub struct AreaEdge {
    pub area1_id: u64,
    pub area2_id: u64,
    pub line: Line,
    pub weight: i32,
}
