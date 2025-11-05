use crate::simulation::state::world::grid;

#[derive(Clone, Debug)]
pub struct Face {
    pub direction: grid::Direction,
    pub light_level: f32,
    pub exposed: bool,
}

impl Face {
    pub fn new(direction: grid::Direction) -> Self {
        Self {
            direction,
            light_level: 1.0,
            exposed: true,
        }
    }
}
