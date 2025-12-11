use crate::simulation::state::world::grid::Axis;
use ultraviolet::IVec3;

#[derive(Clone, Debug)]
pub struct Line {
    pub axis: Axis,
    pub grid_position1: IVec3,
    pub grid_position2: IVec3,
}

impl Line {
    pub fn new(grid_position1: IVec3, grid_position2: IVec3) -> Self {
        let axis = if grid_position1.y == grid_position2.y && grid_position1.z == grid_position2.z {
            Axis::X
        } else if grid_position1.z == grid_position2.z && grid_position1.x == grid_position2.x {
            Axis::Y
        } else if grid_position1.x == grid_position2.x && grid_position1.y == grid_position2.y {
            Axis::Z
        } else {
            panic!("Line is not axis-aligned!");
        };

        Self {
            axis,
            grid_position1,
            grid_position2,
        }
    }

    pub fn midpoint(line: &Self) -> IVec3 {
        (line.grid_position1 + line.grid_position2) / 2
    }
}
