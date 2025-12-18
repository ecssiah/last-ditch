use crate::simulation::state::physics::collider::Collider;
use std::collections::HashMap;
use ultraviolet::Vec3;

pub struct CompoundBody {
    pub active: bool,
    pub world_position: Vec3,
    pub collider_vec: Vec<Collider>,
    pub collider_map: HashMap<String, usize>,
}

impl CompoundBody {
    pub fn new() -> Self {
        Self {
            active: true,
            world_position: Vec3::zero(),
            collider_vec: Vec::new(),
            collider_map: HashMap::new(),
        }
    }

    pub fn add(name: String, collider: Collider, body: &mut Self) {
        body.collider_vec.push(collider);
        body.collider_map.insert(name, body.collider_vec.len() - 1);
    }

    pub fn set_world_position(world_position: Vec3, body: &mut Self) {
        for collider in body.collider_vec.iter_mut() {
            Collider::set_world_position(collider.relative_position + world_position, collider);
        }
    }

    pub fn translate(displacement: Vec3, body: &mut Self) {
        body.world_position += displacement;

        for collider in body.collider_vec.iter_mut() {
            Collider::set_world_position(
                collider.relative_position + body.world_position,
                collider,
            );
        }
    }
}
