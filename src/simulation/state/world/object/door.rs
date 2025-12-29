pub mod kind;

pub use kind::Kind;

use crate::simulation::{
    constants::{CELL_RADIUS_IN_METERS, CELL_SIZE_IN_METERS},
    state::{
        physics::collider::box_collider::BoxCollider,
        world::{grid::Direction, object},
    },
};
use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct Door {
    pub door_kind: self::Kind,
    pub direction: Direction,
    pub box_collider: BoxCollider,
    pub open: bool,
    pub locked: bool,
}

impl Door {
    pub fn new(door_kind: &self::Kind, direction: &Direction) -> Self {
        let door_info = Self::get_door_info(door_kind);
        let box_collider = BoxCollider::new(door_info.local_position, door_info.radius);

        Self {
            door_kind: door_kind.clone(),
            direction: direction.clone(),
            box_collider,
            open: true,
            locked: false,
        }
    }

    pub fn get_door_info(door_kind: &self::Kind) -> object::Info {
        match door_kind {
            Kind::Door1 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, CELL_RADIUS_IN_METERS),
                radius: Vec3::new(1.00, 0.25, 2.00) * CELL_SIZE_IN_METERS,
            },
        }
    }
}
