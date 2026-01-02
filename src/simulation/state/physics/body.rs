pub mod body_label;
pub mod contact;
pub mod contact_set;

pub use contact::Contact;
pub use contact_set::ContactSet;

use crate::{
    simulation::state::physics::{body::body_label::BodyLabel, collider::Collider},
    utils::ldmath::FloatBox,
};
use std::collections::HashMap;
use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct Body {
    pub world_position: Vec3,
    pub radius: Vec3,
    pub collider_map: HashMap<BodyLabel, usize>,
    pub collider_vec: Vec<Collider>,
    pub contact_set: ContactSet,
}

impl Body {
    pub fn new() -> Self {
        let world_position = Vec3::zero();
        let radius = Vec3::zero();
        let collider_map = HashMap::new();
        let collider_vec = Vec::new();
        let contact_set = ContactSet::EMPTY;

        Self {
            world_position,
            radius,
            collider_map,
            collider_vec,
            contact_set,
        }
    }

    pub fn set_world_position(world_position: Vec3, body: &mut Self) {
        body.world_position = world_position;

        for &collider_index in body.collider_map.values() {
            if let Some(collider) = body.collider_vec.get_mut(collider_index) {
                Collider::set_world_position(world_position + collider.local_position, collider);
            }
        }
    }

    pub fn add_collider(
        body_label: &BodyLabel,
        local_position: Vec3,
        radius: Vec3,
        body: &mut Self,
    ) {
        let collider = Collider::new(body.world_position + local_position, local_position, radius);

        body.collider_vec.push(collider);

        let collider_index = body.collider_vec.len() - 1;

        body.collider_map.insert(body_label.clone(), collider_index);

        let radius = Self::calculate_radius(body);

        body.radius = radius;
    }

    pub fn set_collider_local_position(
        body_label: &BodyLabel,
        local_position: Vec3,
        body: &mut Self,
    ) {
        if let Some(&collider_index) = body.collider_map.get(body_label) {
            if let Some(collider) = body.collider_vec.get_mut(collider_index) {
                collider.local_position = local_position;

                Collider::set_world_position(body.world_position + local_position, collider);

                let radius = Self::calculate_radius(body);

                body.radius = radius;
            }
        }
    }

    pub fn set_collider_radius(body_label: &BodyLabel, radius: Vec3, body: &mut Self) {
        if let Some(&collider_index) = body.collider_map.get(body_label) {
            if let Some(collider) = body.collider_vec.get_mut(collider_index) {
                Collider::set_radius(radius, collider);

                let radius = Self::calculate_radius(body);

                body.radius = radius;
            }
        }
    }

    fn calculate_radius(body: &Self) -> Vec3 {
        if body.collider_vec.is_empty() {
            return Vec3::zero();
        }

        let mut min = Vec3::broadcast(f32::INFINITY);
        let mut max = Vec3::broadcast(f32::NEG_INFINITY);

        for collider in &body.collider_vec {
            let collider_min = FloatBox::get_min(&collider.float_box);
            let collider_max = FloatBox::get_max(&collider.float_box);

            min = min.min_by_component(collider_min);
            max = max.max_by_component(collider_max);
        }

        let bounding_box = FloatBox::from_bounds(min, max);

        FloatBox::get_radius(&bounding_box)
    }
}

impl Default for Body {
    fn default() -> Self {
        Self::new()
    }
}
