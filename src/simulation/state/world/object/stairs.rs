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
    pub direction: Direction,
    pub wedge_collider: WedgeCollider,
}

impl Stairs {
    pub fn get_door_info(door_kind: &self::Kind) -> object::Info {
        match door_kind {
            Kind::Stairs1 => object::Info {
                solid: true,
                local_position: Vec3::new(0.0, 0.0, 0.0),
                radius: Vec3::new(1.00, 1.00, 1.00) * CELL_RADIUS_IN_METERS,
            },
        }
    }
}
