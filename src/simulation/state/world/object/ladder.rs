pub mod kind;

pub use kind::Kind;

use ultraviolet::Vec3;
use crate::simulation::{
    constants::{CELL_RADIUS_IN_METERS, CELL_UNIT_EIGHTH},
    state::{
        physics::collider::box_collider::BoxCollider,
        world::{grid::Direction, object},
    },
};

#[derive(Clone, Debug)]
pub struct Ladder {
    pub ladder_kind: self::Kind,
    pub direction: Direction,
    pub box_collider: BoxCollider,
}

impl Ladder {
    pub fn get_door_info(door_kind: &self::Kind) -> object::Info {
        match door_kind {
            Kind::Ladder1 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, CELL_RADIUS_IN_METERS - CELL_UNIT_EIGHTH, 0.0),
                radius: Vec3::new(1.00, 0.25, 1.00) * CELL_RADIUS_IN_METERS,
            },
        }
    }
}
