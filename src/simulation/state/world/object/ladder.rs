pub mod kind;

pub use kind::Kind;

use crate::simulation::{
    constants::{CELL_RADIUS_IN_METERS, CELL_UNIT_EIGHTH},
    state::{
        physics::collider::box_collider::BoxCollider,
        world::{grid::Direction, object},
    },
};
use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct Ladder {
    pub ladder_kind: self::Kind,
    pub direction: Direction,
    pub box_collider: BoxCollider,
}

impl Ladder {
    pub fn new(ladder_kind: &self::Kind, direction: &Direction) -> Self {
        let ladder_info = Self::get_ladder_info(ladder_kind);
        let box_collider = BoxCollider::new(ladder_info.local_position, ladder_info.radius);

        Self {
            ladder_kind: ladder_kind.clone(),
            direction: direction.clone(),
            box_collider,
        }
    }

    pub fn get_ladder_info(ladder_kind: &self::Kind) -> object::Info {
        match ladder_kind {
            Kind::Ladder1 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, CELL_RADIUS_IN_METERS - CELL_UNIT_EIGHTH, 0.0),
                radius: Vec3::new(1.00, 0.25, 1.00) * CELL_RADIUS_IN_METERS,
            },
        }
    }
}
