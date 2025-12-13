use crate::simulation::state::world::grid::{Direction, Quadrant};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Kind {
    Center,
    CenterHall(Direction),
    OuterHall(Direction),
    CornerHall(Quadrant),
    Room,
}
