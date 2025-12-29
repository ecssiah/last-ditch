pub mod contact;
pub mod contact_set;

pub use contact::Contact;
pub use contact_set::ContactSet;

use crate::simulation::{
    constants::CELL_RADIUS_IN_METERS,
    state::physics::collider::{self, box_collider::BoxCollider},
};
use std::collections::HashMap;
use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct Body {
    pub contact_set: ContactSet,
    pub box_collider_vec: Vec<BoxCollider>,
    pub box_collider_index_map: HashMap<collider::Label, usize>,
}

impl Body {
    pub fn new(radius: Vec3) -> Self {
        let contact_set = ContactSet::EMPTY;
        let box_collider_vec = vec![BoxCollider::new(Vec3::zero(), radius)];
        let box_collider_index_map =
            HashMap::from([(collider::Label::Core, box_collider_vec.len() - 1)]);

        Self {
            contact_set,
            box_collider_vec,
            box_collider_index_map,
        }
    }

    pub fn add_collider(
        collider_label: &collider::Label,
        box_collider: BoxCollider,
        body: &mut Self,
    ) {
        body.box_collider_vec.push(box_collider);

        body.box_collider_index_map
            .insert(collider_label.clone(), body.box_collider_vec.len() - 1);
    }

    pub fn get_collider(collider_label: collider::Label, body: &Self) -> Option<&BoxCollider> {
        let collider_index = body.box_collider_index_map.get(&collider_label)?;

        body.box_collider_vec.get(*collider_index)
    }

    pub fn get_collider_mut(
        collider_label: collider::Label,
        body: &mut Self,
    ) -> Option<&mut BoxCollider> {
        let collider_index = body.box_collider_index_map.get(&collider_label)?;

        body.box_collider_vec.get_mut(*collider_index)
    }

    pub fn set_world_position(world_position: Vec3, body: &mut Self) {
        for box_collider in &mut body.box_collider_vec {
            let collider_world_position = world_position + box_collider.local_position;

            BoxCollider::set_world_position(collider_world_position, box_collider);
        }
    }
}

impl Default for Body {
    fn default() -> Self {
        Self::new(Vec3::broadcast(CELL_RADIUS_IN_METERS))
    }
}
