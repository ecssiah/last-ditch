use crate::{simulation::state::physics::collider::Collider, utils::ldmath::FBox};
use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct SimpleBody {
    pub active: bool,
    pub world_position: Vec3,
    pub collider: Collider,
}

impl SimpleBody {
    pub fn new(world_position: Vec3, size: Vec3) -> Self {
        Self {
            active: true,
            world_position,
            collider: Collider::new(world_position, size),
        }
    }

    pub fn default() -> Self {
        Self::new(Vec3::zero(), Vec3::one())
    }

    pub fn get_cell_bounding_box(body: &Self) -> FBox {
        let size = body.collider.size * 0.5;

        FBox {
            min: body.world_position + body.collider.relative_position - size,
            max: body.world_position + body.collider.relative_position + size,
        }
    }

    pub fn set_world_position(world_position: Vec3, body: &mut Self) {
        Collider::set_world_position(
            body.collider.relative_position + world_position,
            &mut body.collider,
        );
    }

    pub fn set_size(size: Vec3, body: &mut Self) {
        Collider::set_size(size, &mut body.collider);
    }

    pub fn translate(displacement: Vec3, body: &mut Self) {
        body.world_position += displacement;

        Collider::set_world_position(
            body.collider.relative_position + body.world_position,
            &mut body.collider,
        );
    }
}
