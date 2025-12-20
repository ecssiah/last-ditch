pub mod info;
pub mod kind;
pub mod label;
pub mod owner;

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
    pub fn new(local_position: Vec3, size: Vec3) -> Self {
        let active = true;
        let collider_kind = collider::Kind::Solid;
        let float_box = FloatBox::new(-size * 0.5, size * 0.5);

        Self {
            active,
            local_position,
            collider_kind,
            float_box,
        }
    }

    pub fn default() -> Self {
        Self::new(Vec3::zero(), Vec3::broadcast(CELL_RADIUS_IN_METERS))
    }

    pub fn set_local_position(local_position: Vec3, collider: &mut Self) {
        collider.local_position = local_position;
    }

    pub fn get_world_position(collider: &Self) -> Vec3 {
        (collider.float_box.min + collider.float_box.max) * 0.5
    }

    pub fn set_world_position(world_position: Vec3, collider: &mut Self) {
        let collider_radius = FloatBox::get_radius(&collider.float_box);

        collider.float_box.min = world_position - collider_radius;
        collider.float_box.max = world_position + collider_radius;
    }

    pub fn update_world_position(parent_world_position: Vec3, collider: &mut Self) {
        let collider_world_position = parent_world_position + collider.local_position;
        let collider_radius = FloatBox::get_radius(&collider.float_box);

        collider.float_box.min = collider_world_position - collider_radius;
        collider.float_box.max = collider_world_position + collider_radius;
    }

    pub fn get_size(collider: &Self) -> Vec3 {
        collider.float_box.max - collider.float_box.min
    }

    pub fn set_size(size: Vec3, collider: &mut Self) {
        let collider_world_position = Self::get_world_position(collider);
        let collider_radius = size * 0.5;

        collider.float_box.min = collider_world_position - collider_radius;
        collider.float_box.max = collider_world_position + collider_radius;
    }
}
