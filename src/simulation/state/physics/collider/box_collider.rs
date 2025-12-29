use ultraviolet::Vec3;
use crate::{simulation::constants::CELL_RADIUS_IN_METERS, utils::ldmath::FloatBox};

#[derive(Clone, Debug)]
pub struct BoxCollider {
    pub active: bool,
    pub local_position: Vec3,
    pub float_box: FloatBox,
}

impl BoxCollider {
    pub fn new(local_position: Vec3, radius: Vec3) -> Self {
        let active = true;
        let float_box = FloatBox::new(Vec3::zero(), radius);

        Self {
            active,
            local_position,
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

impl Default for BoxCollider {
    fn default() -> Self {
        Self::new(
            Vec3::zero(),
            Vec3::broadcast(CELL_RADIUS_IN_METERS),
        )
    }
}
