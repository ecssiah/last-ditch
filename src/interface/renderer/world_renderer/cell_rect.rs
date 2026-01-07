use crate::simulation::constants::CELL_RADIUS_IN_METERS;
use ultraviolet::Vec3;

#[derive(Clone, PartialEq)]
pub struct CellRect {
    pub position_array: [Vec3; 4],
    pub normal_array: [Vec3; 4],
}

impl CellRect {
    pub const NORTH_CELL_RECT: CellRect = CellRect {
        position_array: [
            Vec3::new(
                CELL_RADIUS_IN_METERS,
                CELL_RADIUS_IN_METERS,
                -CELL_RADIUS_IN_METERS,
            ),
            Vec3::new(
                CELL_RADIUS_IN_METERS,
                CELL_RADIUS_IN_METERS,
                CELL_RADIUS_IN_METERS,
            ),
            Vec3::new(
                -CELL_RADIUS_IN_METERS,
                CELL_RADIUS_IN_METERS,
                CELL_RADIUS_IN_METERS,
            ),
            Vec3::new(
                -CELL_RADIUS_IN_METERS,
                CELL_RADIUS_IN_METERS,
                -CELL_RADIUS_IN_METERS,
            ),
        ],
        normal_array: [
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ],
    };

    pub const WEST_CELL_RECT: CellRect = CellRect {
        position_array: [
            Vec3::new(
                -CELL_RADIUS_IN_METERS,
                CELL_RADIUS_IN_METERS,
                -CELL_RADIUS_IN_METERS,
            ),
            Vec3::new(
                -CELL_RADIUS_IN_METERS,
                CELL_RADIUS_IN_METERS,
                CELL_RADIUS_IN_METERS,
            ),
            Vec3::new(
                -CELL_RADIUS_IN_METERS,
                -CELL_RADIUS_IN_METERS,
                CELL_RADIUS_IN_METERS,
            ),
            Vec3::new(
                -CELL_RADIUS_IN_METERS,
                -CELL_RADIUS_IN_METERS,
                -CELL_RADIUS_IN_METERS,
            ),
        ],
        normal_array: [
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
        ],
    };

    pub const SOUTH_CELL_RECT: CellRect = CellRect {
        position_array: [
            Vec3::new(
                -CELL_RADIUS_IN_METERS,
                -CELL_RADIUS_IN_METERS,
                -CELL_RADIUS_IN_METERS,
            ),
            Vec3::new(
                -CELL_RADIUS_IN_METERS,
                -CELL_RADIUS_IN_METERS,
                CELL_RADIUS_IN_METERS,
            ),
            Vec3::new(
                CELL_RADIUS_IN_METERS,
                -CELL_RADIUS_IN_METERS,
                CELL_RADIUS_IN_METERS,
            ),
            Vec3::new(
                CELL_RADIUS_IN_METERS,
                -CELL_RADIUS_IN_METERS,
                -CELL_RADIUS_IN_METERS,
            ),
        ],
        normal_array: [
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
        ],
    };

    pub const EAST_CELL_RECT: CellRect = CellRect {
        position_array: [
            Vec3::new(
                CELL_RADIUS_IN_METERS,
                -CELL_RADIUS_IN_METERS,
                -CELL_RADIUS_IN_METERS,
            ),
            Vec3::new(
                CELL_RADIUS_IN_METERS,
                -CELL_RADIUS_IN_METERS,
                CELL_RADIUS_IN_METERS,
            ),
            Vec3::new(
                CELL_RADIUS_IN_METERS,
                CELL_RADIUS_IN_METERS,
                CELL_RADIUS_IN_METERS,
            ),
            Vec3::new(
                CELL_RADIUS_IN_METERS,
                CELL_RADIUS_IN_METERS,
                -CELL_RADIUS_IN_METERS,
            ),
        ],
        normal_array: [
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
        ],
    };

    pub const UP_CELL_RECT: CellRect = CellRect {
        position_array: [
            Vec3::new(
                CELL_RADIUS_IN_METERS,
                -CELL_RADIUS_IN_METERS,
                CELL_RADIUS_IN_METERS,
            ),
            Vec3::new(
                CELL_RADIUS_IN_METERS,
                CELL_RADIUS_IN_METERS,
                CELL_RADIUS_IN_METERS,
            ),
            Vec3::new(
                -CELL_RADIUS_IN_METERS,
                CELL_RADIUS_IN_METERS,
                CELL_RADIUS_IN_METERS,
            ),
            Vec3::new(
                -CELL_RADIUS_IN_METERS,
                -CELL_RADIUS_IN_METERS,
                CELL_RADIUS_IN_METERS,
            ),
        ],
        normal_array: [
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 1.0),
        ],
    };

    pub const DOWN_CELL_RECT: CellRect = CellRect {
        position_array: [
            Vec3::new(
                -CELL_RADIUS_IN_METERS,
                CELL_RADIUS_IN_METERS,
                -CELL_RADIUS_IN_METERS,
            ),
            Vec3::new(
                -CELL_RADIUS_IN_METERS,
                -CELL_RADIUS_IN_METERS,
                -CELL_RADIUS_IN_METERS,
            ),
            Vec3::new(
                CELL_RADIUS_IN_METERS,
                -CELL_RADIUS_IN_METERS,
                -CELL_RADIUS_IN_METERS,
            ),
            Vec3::new(
                CELL_RADIUS_IN_METERS,
                CELL_RADIUS_IN_METERS,
                -CELL_RADIUS_IN_METERS,
            ),
        ],
        normal_array: [
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 0.0, -1.0),
        ],
    };
}
