pub mod collider_kind;

pub use collider_kind::ColliderKind;

use crate::utils::ldmath::FloatBox;
use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct Collider {
    pub local_position: Vec3,
    pub float_box: FloatBox,
}

impl Collider {
    pub fn new(world_position: Vec3, local_position: Vec3, radius: Vec3) -> Self {
        Self {
            local_position,
            float_box: FloatBox::new(world_position, radius),
        }
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
}
