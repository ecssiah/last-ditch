use crate::simulation::{constants::*, state::world::grid::Quadrant};
use ultraviolet::IVec3;

pub struct Tower {}

impl Tower {
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

    pub fn get_quadrant_min(quadrant: Quadrant, floor_number: i32) -> IVec3 {
        let tower_radius = TOWER_RADIUS as i32;
        let tower_floor_height = TOWER_FLOOR_HEIGHT as i32;

        let floor_z_min = floor_number * tower_floor_height;

        match quadrant {
            Quadrant::NE => IVec3::new(1, 1, floor_z_min),
            Quadrant::NW => IVec3::new(-tower_radius, 1, floor_z_min),
            Quadrant::SW => IVec3::new(-tower_radius, -tower_radius, floor_z_min),
            Quadrant::SE => IVec3::new(1, -tower_radius, floor_z_min),
        }
    }

    pub fn get_quadrant_max(quadrant: Quadrant, floor_number: i32) -> IVec3 {
        let tower_radius = TOWER_RADIUS as i32;
        let tower_floor_height = TOWER_FLOOR_HEIGHT as i32;

        let floor_z_max = floor_number * tower_floor_height + tower_floor_height - 1;

        match quadrant {
            Quadrant::NE => IVec3::new(tower_radius, tower_radius, floor_z_max),
            Quadrant::NW => IVec3::new(-1, tower_radius, floor_z_max),
            Quadrant::SW => IVec3::new(-1, -1, floor_z_max),
            Quadrant::SE => IVec3::new(tower_radius, -1, floor_z_max),
        }
    }
}
