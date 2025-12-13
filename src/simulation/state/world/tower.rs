pub mod floor;

pub use floor::Floor;

use crate::simulation::{
    constants::*,
    state::world::grid::{self, Quadrant},
};
use std::collections::HashMap;
use ultraviolet::IVec3;

pub struct Tower {
    pub floor_map: HashMap<i32, Floor>,
}

impl Tower {
    pub fn new() -> Self {
        let floor_map = HashMap::new();

        Self { floor_map }
    }

    pub fn reset(tower: &mut Tower) {
        tower.floor_map.clear();
    }

    pub fn get_floor_z_min(floor_number: i32) -> i32 {
        let tower_floor_height = TOWER_FLOOR_HEIGHT as i32;

        let floor_z_min = floor_number * tower_floor_height;

        floor_z_min
    }

    pub fn get_floor_z_max(floor_number: i32) -> i32 {
        let tower_floor_height = TOWER_FLOOR_HEIGHT as i32;

        let floor_z_max = floor_number * tower_floor_height + tower_floor_height - 1;

        floor_z_max
    }

    pub fn get_floor_min(floor_number: i32) -> IVec3 {
        let tower_radius = TOWER_RADIUS as i32;

        let floor_min = IVec3::new(
            -tower_radius,
            -tower_radius,
            Self::get_floor_z_min(floor_number),
        );

        floor_min
    }

    pub fn get_floor_max(floor_number: i32) -> IVec3 {
        let tower_radius = TOWER_RADIUS as i32;

        let floor_max = IVec3::new(
            tower_radius,
            tower_radius,
            Self::get_floor_z_max(floor_number),
        );

        floor_max
    }

    pub fn get_center_min(floor_number: i32) -> IVec3 {
        let tower_central_hall_radius = TOWER_CENTER_HALL_RADIUS as i32;

        IVec3::new(
            -tower_central_hall_radius,
            -tower_central_hall_radius,
            Self::get_floor_z_min(floor_number),
        )
    }

    pub fn get_center_max(floor_number: i32) -> IVec3 {
        let tower_central_hall_radius = TOWER_CENTER_HALL_RADIUS as i32;

        IVec3::new(
            tower_central_hall_radius,
            tower_central_hall_radius,
            Self::get_floor_z_max(floor_number),
        )
    }

    pub fn get_quadrant_min(quadrant: Quadrant, floor_number: i32) -> IVec3 {
        let tower_radius = TOWER_RADIUS as i32;
        let tower_central_hall_radius = TOWER_CENTER_HALL_RADIUS as i32;
        let tower_outer_hall_size = TOWER_OUTER_HALL_SIZE as i32;

        let floor_min = Self::get_floor_min(floor_number);

        match quadrant {
            Quadrant::NE => IVec3::new(
                tower_central_hall_radius,
                tower_central_hall_radius,
                floor_min.z,
            ),
            Quadrant::NW => IVec3::new(
                -tower_radius + tower_outer_hall_size,
                tower_central_hall_radius,
                floor_min.z,
            ),
            Quadrant::SW => IVec3::new(
                -tower_radius + tower_outer_hall_size,
                -tower_radius + tower_outer_hall_size,
                floor_min.z,
            ),
            Quadrant::SE => IVec3::new(
                tower_central_hall_radius,
                -tower_radius + tower_outer_hall_size,
                floor_min.z,
            ),
        }
    }

    pub fn get_quadrant_max(quadrant: Quadrant, floor_number: i32) -> IVec3 {
        let tower_radius = TOWER_RADIUS as i32;
        let tower_central_hall_radius = TOWER_CENTER_HALL_RADIUS as i32;
        let tower_outer_hall_size = TOWER_OUTER_HALL_SIZE as i32;

        let floor_max = Self::get_floor_max(floor_number);

        match quadrant {
            Quadrant::NE => IVec3::new(
                tower_radius - tower_outer_hall_size,
                tower_radius - tower_outer_hall_size,
                floor_max.z,
            ),
            Quadrant::NW => IVec3::new(
                -tower_central_hall_radius,
                tower_radius - tower_outer_hall_size,
                floor_max.z,
            ),
            Quadrant::SW => IVec3::new(
                -tower_central_hall_radius,
                -tower_central_hall_radius,
                floor_max.z,
            ),
            Quadrant::SE => IVec3::new(
                tower_radius - tower_outer_hall_size,
                -tower_central_hall_radius,
                floor_max.z,
            ),
        }
    }

    pub fn get_corner_hall_min(quadrant: Quadrant, floor_number: i32) -> IVec3 {
        let tower_radius = TOWER_RADIUS as i32;
        let tower_outer_hall_size = TOWER_OUTER_HALL_SIZE as i32;

        let floor_min = Self::get_floor_min(floor_number);

        match quadrant {
            Quadrant::NE => IVec3::new(
                tower_radius - tower_outer_hall_size,
                tower_radius - tower_outer_hall_size,
                floor_min.z,
            ),
            Quadrant::NW => IVec3::new(
                -tower_radius,
                tower_radius - tower_outer_hall_size,
                floor_min.z,
            ),
            Quadrant::SW => IVec3::new(-tower_radius, -tower_radius, floor_min.z),
            Quadrant::SE => IVec3::new(
                tower_radius - tower_outer_hall_size,
                -tower_radius,
                floor_min.z,
            ),
        }
    }

    pub fn get_corner_hall_max(quadrant: Quadrant, floor_number: i32) -> IVec3 {
        let tower_radius = TOWER_RADIUS as i32;
        let tower_outer_hall_size = TOWER_OUTER_HALL_SIZE as i32;

        let floor_max = Self::get_floor_max(floor_number);

        match quadrant {
            Quadrant::NE => IVec3::new(tower_radius, tower_radius, floor_max.z),
            Quadrant::NW => IVec3::new(
                -tower_radius + tower_outer_hall_size,
                tower_radius,
                floor_max.z,
            ),
            Quadrant::SW => IVec3::new(
                -tower_radius + tower_outer_hall_size,
                -tower_radius + tower_outer_hall_size,
                floor_max.z,
            ),
            Quadrant::SE => IVec3::new(
                tower_radius,
                -tower_radius + tower_outer_hall_size,
                floor_max.z,
            ),
        }
    }

    pub fn get_outer_hall_min(direction: grid::Direction, floor_number: i32) -> IVec3 {
        let tower_radius = TOWER_RADIUS as i32;
        let tower_outer_hall_size = TOWER_OUTER_HALL_SIZE as i32;

        let floor_min = Self::get_floor_min(floor_number);

        match direction {
            grid::Direction::East => IVec3::new(
                tower_radius - tower_outer_hall_size,
                -tower_radius + tower_outer_hall_size,
                floor_min.z,
            ),
            grid::Direction::West => IVec3::new(
                -tower_radius,
                -tower_radius + tower_outer_hall_size,
                floor_min.z,
            ),
            grid::Direction::North => IVec3::new(
                -tower_radius + tower_outer_hall_size,
                tower_radius - tower_outer_hall_size,
                floor_min.z,
            ),
            grid::Direction::South => IVec3::new(
                -tower_radius + tower_outer_hall_size,
                -tower_radius,
                floor_min.z,
            ),
            _ => panic!("No external hall in this direction!"),
        }
    }

    pub fn get_outer_hall_max(direction: grid::Direction, floor_number: i32) -> IVec3 {
        let tower_radius = TOWER_RADIUS as i32;
        let tower_outer_hall_size = TOWER_OUTER_HALL_SIZE as i32;

        let floor_max = Self::get_floor_max(floor_number);

        match direction {
            grid::Direction::East => IVec3::new(
                tower_radius,
                tower_radius - tower_outer_hall_size,
                floor_max.z,
            ),
            grid::Direction::West => IVec3::new(
                -tower_radius,
                tower_radius - tower_outer_hall_size,
                floor_max.z,
            ),
            grid::Direction::North => IVec3::new(
                tower_radius - tower_outer_hall_size,
                tower_radius,
                floor_max.z,
            ),
            grid::Direction::South => IVec3::new(
                tower_radius - tower_outer_hall_size,
                -tower_radius + tower_outer_hall_size,
                floor_max.z,
            ),
            _ => panic!("No external hall in this direction!"),
        }
    }

    pub fn get_center_hall_min(direction: grid::Direction, floor_number: i32) -> IVec3 {
        let tower_radius = TOWER_RADIUS as i32;
        let tower_central_hall_radius = TOWER_CENTER_HALL_RADIUS as i32;
        let tower_outer_hall_size = TOWER_OUTER_HALL_SIZE as i32;

        let floor_min = Self::get_floor_min(floor_number);

        match direction {
            grid::Direction::East => IVec3::new(
                tower_central_hall_radius,
                -tower_central_hall_radius,
                floor_min.z,
            ),
            grid::Direction::West => IVec3::new(
                -tower_radius + tower_outer_hall_size,
                -tower_central_hall_radius,
                floor_min.z,
            ),
            grid::Direction::North => IVec3::new(
                -tower_central_hall_radius,
                tower_central_hall_radius,
                floor_min.z,
            ),
            grid::Direction::South => IVec3::new(
                -tower_central_hall_radius,
                -tower_radius + tower_outer_hall_size,
                floor_min.z,
            ),
            _ => panic!("No internal hall in this direction!"),
        }
    }

    pub fn get_center_hall_max(direction: grid::Direction, floor_number: i32) -> IVec3 {
        let tower_radius = TOWER_RADIUS as i32;
        let tower_central_hall_radius = TOWER_CENTER_HALL_RADIUS as i32;
        let tower_outer_hall_size = TOWER_OUTER_HALL_SIZE as i32;

        let floor_max = Self::get_floor_max(floor_number);

        match direction {
            grid::Direction::East => IVec3::new(
                tower_radius - tower_outer_hall_size,
                tower_central_hall_radius,
                floor_max.z,
            ),
            grid::Direction::West => IVec3::new(
                -tower_central_hall_radius,
                tower_central_hall_radius,
                floor_max.z,
            ),
            grid::Direction::North => IVec3::new(
                tower_central_hall_radius,
                tower_radius - tower_outer_hall_size,
                floor_max.z,
            ),
            grid::Direction::South => IVec3::new(
                tower_central_hall_radius,
                -tower_central_hall_radius,
                floor_max.z,
            ),
            _ => panic!("No internal hall in this direction!"),
        }
    }
}
