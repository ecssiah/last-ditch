use crate::simulation::state::physics::collider::Collider;
use ultraviolet::Vec3;

pub struct PersonBody {
    pub core: Collider,
}

impl PersonBody {
    pub fn new() -> Self {
        Self {
            core: Collider::default(),
        }
    }

    pub fn set_world_position(world_position: Vec3, person_body: &mut Self) {
        person_body.core.set_bottom_center(world_position);
    }

    pub fn set_size(size: Vec3, person_body: &mut Self) {
        person_body.core.set_size(size);
    }
}
