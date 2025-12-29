pub mod kind;

pub use kind::Kind;

use crate::simulation::{
    constants::CELL_RADIUS_IN_METERS,
    state::{
        physics::collider::wedge_collider::WedgeCollider,
        world::{grid::Direction, object},
    },
};
use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct Stairs {
    pub stairs_kind: self::Kind,
    pub direction: Direction,
    pub wedge_collider: WedgeCollider,
}

impl Stairs {
    pub fn new(stairs_kind: &self::Kind, direction: &Direction) -> Self {
        let stairs_info = Self::get_stairs_info(stairs_kind);

        let wedge_collider = WedgeCollider::new(
            stairs_info.local_position,
            direction.clone(),
            stairs_info.radius,
        );

        Self {
            stairs_kind: stairs_kind.clone(),
            direction: direction.clone(),
            wedge_collider,
        }
    }

    pub fn get_stairs_info(stairs_kind: &self::Kind) -> object::Info {
        match stairs_kind {
            Kind::Stairs1 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::new(1.00, 1.00, 1.00) * CELL_RADIUS_IN_METERS,
            },
        }
    }
}
