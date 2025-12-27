pub mod kind;

pub use kind::Kind;

use crate::simulation::{
    constants::*,
    state::{
        physics::collider::{self, Collider},
        world::{grid::Direction, object},
    },
};
use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct Object {
    pub object_kind: object::Kind,
    pub direction: Direction,
    pub collider: Collider,
}

impl Object {
    pub fn new(object_kind: self::Kind, direction: Direction) -> Self {
        Self {
            object_kind,
            direction,
            collider: Self::setup_collider(object_kind, direction),
        }
    }

    fn setup_collider(object_kind: self::Kind, direction: Direction) -> Collider {
        match object_kind {
            Kind::DoorOpen => {
                let local_position = Vec3::new(0.0, 0.0, CELL_RADIUS_IN_METERS);
                let radius = Vec3::new(1.00, 0.25, 2.00) * CELL_SIZE_IN_METERS;

                let mut collider = Collider::new(&collider::Kind::Solid, local_position, radius);
                collider.active = false;

                collider
            }
            Kind::DoorClosed => {
                let local_position = Vec3::new(0.0, 0.0, CELL_RADIUS_IN_METERS);
                let radius = Vec3::new(1.00, 0.25, 2.00) * CELL_RADIUS_IN_METERS;

                let collider = Collider::new(&collider::Kind::Solid, local_position, radius);

                collider
            }
            Kind::Stairs => {
                let local_position = Vec3::new(0.0, 0.0, 0.0);
                let radius = Vec3::new(1.00, 1.00, 1.00) * CELL_RADIUS_IN_METERS;

                let collider_kind = if direction == Direction::North {
                    collider::Kind::StairsNorth
                } else if direction == Direction::West {
                    collider::Kind::StairsWest
                } else if direction == Direction::South {
                    collider::Kind::StairsSouth
                } else if direction == Direction::East {
                    collider::Kind::StairsEast
                } else {
                    panic!("Stairs cannot face up or down")
                };

                let collider = Collider::new(&collider_kind, local_position, radius);

                collider
            }
            Kind::Platform => {
                let local_position = Vec3::new(0.0, 0.0, CELL_RADIUS_IN_METERS - CELL_UNIT_EIGHTH);
                let radius = Vec3::new(1.00, 1.00, 0.25) * CELL_RADIUS_IN_METERS;

                let collider = Collider::new(&collider::Kind::Solid, local_position, radius);

                collider
            }
            Kind::Ladder => {
                let local_position = Vec3::new(0.0, CELL_RADIUS_IN_METERS - CELL_UNIT_EIGHTH, 0.0);
                let radius = Vec3::new(1.00, 0.25, 1.00) * CELL_RADIUS_IN_METERS;

                let collider = Collider::new(&collider::Kind::Ladder, local_position, radius);

                collider
            }
        }
    }
}
