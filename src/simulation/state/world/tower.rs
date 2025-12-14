pub mod floor;

pub use floor::Floor;

use crate::simulation::{
    constants::*,
    state::world::{
        grid::{Direction, Quadrant},
        Area,
    },
};
use std::collections::HashMap;
use ultraviolet::IVec3;

pub struct Tower {
    pub area_map: HashMap<u64, Area>,
    pub floor_map: HashMap<i32, Floor>,
}

impl Tower {
    pub fn new() -> Self {
        let area_map = HashMap::new();
        let floor_map = HashMap::new();

        Self {
            area_map,
            floor_map,
        }
    }

    pub fn reset(tower: &mut Tower) {
        tower.area_map.clear();
        tower.floor_map.clear();
    }

    pub fn get_floor_grid_position(floor_number: i32) -> IVec3 {
        let tower_radius = TOWER_RADIUS as i32;
        let tower_floor_height = TOWER_FLOOR_HEIGHT as i32;

        IVec3::new(
            -tower_radius,
            -tower_radius,
            floor_number * tower_floor_height,
        )
    }

    pub fn get_floor_size() -> IVec3 {
        let tower_radius = TOWER_RADIUS as i32;

        IVec3::new(
            2 * tower_radius + 1,
            2 * tower_radius + 1,
            TOWER_FLOOR_HEIGHT as i32,
        )
    }

    pub fn get_center_grid_position(floor_number: i32) -> IVec3 {
        let tower_floor_height = TOWER_FLOOR_HEIGHT as i32;
        let tower_center_hall_radius = TOWER_CENTER_HALL_RADIUS as i32;

        IVec3::new(
            -tower_center_hall_radius,
            -tower_center_hall_radius,
            floor_number * tower_floor_height,
        )
    }

    pub fn get_center_size() -> IVec3 {
        let tower_floor_height = TOWER_FLOOR_HEIGHT as i32;
        let tower_center_hall_radius = TOWER_CENTER_HALL_RADIUS as i32;

        IVec3::new(
            2 * tower_center_hall_radius + 1,
            2 * tower_center_hall_radius + 1,
            tower_floor_height,
        )
    }

    pub fn get_center_hall_grid_position(direction: Direction, floor_number: i32) -> IVec3 {
        let tower_radius = TOWER_RADIUS as i32;
        let tower_floor_height = TOWER_FLOOR_HEIGHT as i32;
        let tower_center_hall_radius = TOWER_CENTER_HALL_RADIUS as i32;
        let tower_outer_hall_size = TOWER_OUTER_HALL_SIZE as i32;

        match direction {
            Direction::North => IVec3::new(
                -tower_center_hall_radius,
                tower_center_hall_radius,
                floor_number * tower_floor_height,
            ),
            Direction::West => IVec3::new(
                -tower_radius + tower_outer_hall_size - 1,
                -tower_center_hall_radius,
                floor_number * tower_floor_height,
            ),
            Direction::South => IVec3::new(
                -tower_center_hall_radius,
                -tower_radius + tower_outer_hall_size - 1,
                floor_number * tower_floor_height,
            ),
            Direction::East => IVec3::new(
                tower_center_hall_radius,
                -tower_center_hall_radius,
                floor_number * tower_floor_height,
            ),
            _ => panic!("No internal hall in this direction!"),
        }
    }

    pub fn get_center_hall_size(direction: Direction) -> IVec3 {
        let tower_radius = TOWER_RADIUS as i32;
        let tower_floor_height = TOWER_FLOOR_HEIGHT as i32;
        let tower_center_hall_radius = TOWER_CENTER_HALL_RADIUS as i32;
        let tower_outer_hall_size = TOWER_OUTER_HALL_SIZE as i32;

        match direction {
            Direction::North | Direction::South => IVec3::new(
                2 * tower_center_hall_radius + 1,
                tower_radius - tower_outer_hall_size - tower_center_hall_radius + 2,
                tower_floor_height,
            ),
            Direction::East | Direction::West => IVec3::new(
                tower_radius - tower_outer_hall_size - tower_center_hall_radius + 2,
                2 * tower_center_hall_radius + 1,
                tower_floor_height,
            ),
            _ => panic!("No center hall in this direction!"),
        }
    }

    pub fn get_outer_hall_grid_position(direction: Direction, floor_number: i32) -> IVec3 {
        let tower_radius = TOWER_RADIUS as i32;
        let tower_floor_height = TOWER_FLOOR_HEIGHT as i32;
        let tower_outer_hall_size = TOWER_OUTER_HALL_SIZE as i32;

        match direction {
            Direction::North => IVec3::new(
                -tower_radius + tower_outer_hall_size - 1,
                tower_radius - tower_outer_hall_size + 1,
                floor_number * tower_floor_height,
            ),
            Direction::West => IVec3::new(
                -tower_radius,
                -tower_radius + tower_outer_hall_size - 1,
                floor_number * tower_floor_height,
            ),
            Direction::South => IVec3::new(
                -tower_radius + tower_outer_hall_size - 1,
                -tower_radius,
                floor_number * tower_floor_height,
            ),
            Direction::East => IVec3::new(
                tower_radius - tower_outer_hall_size - 1,
                -tower_radius + tower_outer_hall_size + 1,
                floor_number * tower_floor_height,
            ),
            _ => panic!("No external hall in this direction!"),
        }
    }

    pub fn get_outer_hall_size(direction: Direction) -> IVec3 {
        let tower_size = 2 * (TOWER_RADIUS as i32) + 1;
        let tower_floor_height = TOWER_FLOOR_HEIGHT as i32;
        let tower_outer_hall_size = TOWER_OUTER_HALL_SIZE as i32;

        match direction {
            Direction::North | Direction::South => IVec3::new(
                tower_size - 2 * tower_outer_hall_size + 2,
                tower_outer_hall_size,
                tower_floor_height,
            ),
            Direction::East | Direction::West => IVec3::new(
                tower_outer_hall_size,
                tower_size - 2 * tower_outer_hall_size + 2,
                tower_floor_height,
            ),
            _ => panic!("No outer hall in this direction!"),
        }
    }

    pub fn get_corner_hall_grid_position(quadrant: Quadrant, floor_number: i32) -> IVec3 {
        let tower_radius = TOWER_RADIUS as i32;
        let tower_floor_height = TOWER_FLOOR_HEIGHT as i32;
        let tower_outer_hall_size = TOWER_OUTER_HALL_SIZE as i32;

        match quadrant {
            Quadrant::NE => IVec3::new(
                tower_radius - tower_outer_hall_size + 1,
                tower_radius - tower_outer_hall_size + 1,
                floor_number * tower_floor_height,
            ),
            Quadrant::NW => IVec3::new(
                -tower_radius,
                tower_radius - tower_outer_hall_size + 1,
                floor_number * tower_floor_height,
            ),
            Quadrant::SW => IVec3::new(
                -tower_radius,
                -tower_radius,
                floor_number * tower_floor_height,
            ),
            Quadrant::SE => IVec3::new(
                tower_radius - tower_outer_hall_size + 1,
                -tower_radius,
                floor_number * tower_floor_height,
            ),
        }
    }

    pub fn get_corner_hall_size() -> IVec3 {
        let tower_floor_height = TOWER_FLOOR_HEIGHT as i32;
        let tower_outer_hall_size = TOWER_OUTER_HALL_SIZE as i32;

        IVec3::new(
            tower_outer_hall_size,
            tower_outer_hall_size,
            tower_floor_height,
        )
    }

    pub fn get_quadrant_grid_position(quadrant: Quadrant, floor_number: i32) -> IVec3 {
        let tower_radius = TOWER_RADIUS as i32;
        let tower_floor_height = TOWER_FLOOR_HEIGHT as i32;
        let tower_center_hall_radius = TOWER_CENTER_HALL_RADIUS as i32;
        let tower_outer_hall_size = TOWER_OUTER_HALL_SIZE as i32;

        match quadrant {
            Quadrant::NW => IVec3::new(
                -tower_radius + tower_outer_hall_size - 1,
                tower_center_hall_radius,
                floor_number * tower_floor_height,
            ),
            Quadrant::SW => IVec3::new(
                -tower_radius + tower_outer_hall_size - 1,
                -tower_radius + tower_outer_hall_size - 1,
                floor_number * tower_floor_height,
            ),
            Quadrant::SE => IVec3::new(
                tower_center_hall_radius,
                -tower_radius + tower_outer_hall_size - 1,
                floor_number * tower_floor_height,
            ),
            Quadrant::NE => IVec3::new(
                tower_center_hall_radius,
                tower_center_hall_radius,
                floor_number * tower_floor_height,
            ),
        }
    }

    pub fn get_quadrant_size() -> IVec3 {
        let tower_radius = TOWER_RADIUS as i32;
        let tower_floor_height = TOWER_FLOOR_HEIGHT as i32;
        let tower_center_hall_radius = TOWER_CENTER_HALL_RADIUS as i32;
        let tower_outer_hall_size = TOWER_OUTER_HALL_SIZE as i32;

        IVec3::new(
            tower_radius - tower_outer_hall_size - tower_center_hall_radius + 2,
            tower_radius - tower_outer_hall_size - tower_center_hall_radius + 2,
            tower_floor_height,
        )
    }
}
