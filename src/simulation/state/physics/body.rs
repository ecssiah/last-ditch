use ultraviolet::Vec3;

use crate::simulation::state::physics::collider::{self, Collider};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Body {
    pub collider_vec: Vec<Collider>,
    pub collider_index_map: HashMap<collider::Label, usize>,
}

impl Body {
    pub fn new() -> Self {
        let collider_vec = vec![Collider::default()];
        let collider_index_map = HashMap::from([(collider::Label::Core, collider_vec.len() - 1)]);

        Self {
            collider_vec,
            collider_index_map,
        }
    }

    pub fn get_collider<'a>(
        collider_label: collider::Label,
        body: &'a Self,
    ) -> Option<&'a Collider> {
        let collider_index = body.collider_index_map.get(&collider_label)?;

        body.collider_vec.get(*collider_index)
    }

    pub fn get_collider_mut<'a>(
        collider_label: collider::Label,
        body: &'a mut Self,
    ) -> Option<&'a mut Collider> {
        let collider_index = body.collider_index_map.get(&collider_label)?;

        body.collider_vec.get_mut(*collider_index)
    }

    pub fn set_world_position(world_position: Vec3, body: &mut Self) {
        for collider in &mut body.collider_vec {
            let collider_world_position = world_position + collider.local_position;

            Collider::set_world_position(collider_world_position, collider);
        }
    }
}

impl Default for Body {
    fn default() -> Self {
        Self::new()
    }
}
