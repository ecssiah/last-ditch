pub mod hit;
pub mod info;
pub mod kind;
pub mod label;
pub mod owner;

pub use hit::Hit;
pub use info::Info;
pub use kind::Kind;
pub use label::Label;
pub use owner::Owner;

use crate::{
    simulation::{constants::CELL_RADIUS_IN_METERS, state::physics::collider},
    utils::ldmath::FloatBox,
};
use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct Collider {
    pub active: bool,
    pub collider_kind: collider::Kind,
    pub local_position: Vec3,
    pub float_box: FloatBox,
}

impl Collider {
    pub fn new(collider_kind: &collider::Kind, local_position: Vec3, radius: Vec3) -> Self {
        let active = true;
        let collider_kind = collider_kind.clone();
        let float_box = FloatBox::new(Vec3::zero(), radius);

        Self {
            active,
            local_position,
            collider_kind,
            float_box,
        }
    }

    pub fn set_local_position(local_position: Vec3, collider: &mut Self) {
        collider.local_position = local_position;
    }

    pub fn get_world_position(collider: &Self) -> Vec3 {
        FloatBox::get_world_position(&collider.float_box)
    }

    pub fn set_world_position(world_position: Vec3, collider: &mut Self) {
        FloatBox::set_world_position(world_position, &mut collider.float_box);
    }

    pub fn get_radius(collider: &Self) -> Vec3 {
        FloatBox::get_radius(&collider.float_box)
    }

    pub fn set_radius(radius: Vec3, collider: &mut Self) {
        FloatBox::set_radius(radius, &mut collider.float_box);
    }

    pub fn get_size(collider: &Self) -> Vec3 {
        FloatBox::get_size(&collider.float_box)
    }

    pub fn set_size(size: Vec3, collider: &mut Self) {
        let radius = size * 0.5;

        FloatBox::set_radius(radius, &mut collider.float_box);
    }
}

impl Default for Collider {
    fn default() -> Self {
        Self::new(
            &collider::Kind::Solid,
            Vec3::zero(),
            Vec3::broadcast(CELL_RADIUS_IN_METERS),
        )
    }
}
