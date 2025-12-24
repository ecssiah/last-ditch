pub mod kind;

pub use kind::Kind;

use crate::simulation::{
    constants::*,
    state::{
        physics::collider::{self, Collider},
        world::{
            grid::{self, Direction},
            object,
        },
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
    pub fn new(object_kind: self::Kind) -> Self {
        Self {
            object_kind,
            direction: grid::Direction::North,
            collider: Self::setup_collider(object_kind),
        }
    }

    fn setup_collider(object_kind: self::Kind) -> Collider {
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

                let collider = Collider::new(&collider::Kind::Stairs, local_position, radius);

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
