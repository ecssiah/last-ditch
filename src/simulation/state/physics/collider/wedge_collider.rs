use crate::{
    simulation::{constants::CELL_RADIUS_IN_METERS, state::world::grid::Direction},
    utils::ldmath::FloatBox,
};
use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct WedgeCollider {
    pub active: bool,
    pub local_position: Vec3,
    pub direction: Direction,
    pub float_box: FloatBox,
}

impl WedgeCollider {
    pub fn new(local_position: Vec3, direction: Direction, radius: Vec3) -> Self {
        let active = true;
        let float_box = FloatBox::new(Vec3::zero(), radius);

        Self {
            active,
            local_position,
            direction,
            float_box,
        }
    }

    pub fn set_local_position(local_position: Vec3, wedge_collider: &mut Self) {
        wedge_collider.local_position = local_position;
    }

    pub fn get_world_position(wedge_collider: &Self) -> Vec3 {
        FloatBox::get_world_position(&wedge_collider.float_box)
    }

    pub fn set_world_position(world_position: Vec3, wedge_collider: &mut Self) {
        FloatBox::set_world_position(world_position, &mut wedge_collider.float_box);
    }

    pub fn get_radius(wedge_collider: &Self) -> Vec3 {
        FloatBox::get_radius(&wedge_collider.float_box)
    }

    pub fn set_radius(radius: Vec3, wedge_collider: &mut Self) {
        FloatBox::set_radius(radius, &mut wedge_collider.float_box);
    }

    pub fn get_size(wedge_collider: &Self) -> Vec3 {
        FloatBox::get_size(&wedge_collider.float_box)
    }

    pub fn set_size(size: Vec3, wedge_collider: &mut Self) {
        let radius = size * 0.5;

        FloatBox::set_radius(radius, &mut wedge_collider.float_box);
    }
}

impl Default for WedgeCollider {
    fn default() -> Self {
        Self::new(
            Vec3::zero(),
            Direction::North,
            Vec3::broadcast(CELL_RADIUS_IN_METERS),
        )
    }
}
