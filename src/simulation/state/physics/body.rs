pub mod contact;
pub mod contact_set;

pub use contact::Contact;
pub use contact_set::ContactSet;

use crate::simulation::{
    constants::CELL_RADIUS_IN_METERS,
    state::physics::collider::{self, Collider},
};
use std::collections::HashMap;
use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct Body {
    pub is_massive: bool,
    pub contact_set: ContactSet,
    pub collider_vec: Vec<Collider>,
    pub collider_index_map: HashMap<collider::Label, usize>,
}

impl Body {
    pub fn new(size: Vec3) -> Self {
        let is_massive = true;
        let contact_set = ContactSet::EMPTY;
        let collider_vec = vec![Collider::new(&collider::Kind::Solid, Vec3::zero(), size)];
        let collider_index_map = HashMap::from([(collider::Label::Core, collider_vec.len() - 1)]);

        Self {
            is_massive,
            contact_set,
            collider_vec,
            collider_index_map,
        }
    }

    pub fn add_collider(collider_label: &collider::Label, collider: Collider, body: &mut Self) {
        body.collider_vec.push(collider);

        body.collider_index_map
            .insert(collider_label.clone(), body.collider_vec.len() - 1);
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
        Self::new(Vec3::broadcast(CELL_RADIUS_IN_METERS))
    }
}
