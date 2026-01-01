pub mod body_label;
pub mod contact;
pub mod contact_set;

pub use contact::Contact;
pub use contact_set::ContactSet;

use crate::{
    simulation::{constants::CELL_RADIUS_IN_METERS, state::physics::body::body_label::BodyLabel},
    utils::ldmath::FloatBox,
};
use std::collections::HashMap;
use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct Body {
    pub contact_set: ContactSet,
    pub float_box_vec: Vec<FloatBox>,
    pub float_box_index_map: HashMap<BodyLabel, usize>,
}

impl Body {
    pub fn new(radius: Vec3) -> Self {
        let contact_set = ContactSet::EMPTY;
        let float_box_vec = vec![FloatBox::new(Vec3::zero(), radius)];
        let float_box_index_map = HashMap::from([(BodyLabel::Core, float_box_vec.len() - 1)]);

        Self {
            contact_set,
            float_box_vec,
            float_box_index_map,
        }
    }

    pub fn add_float_box(float_box: FloatBox, body_label: &BodyLabel, body: &mut Self) {
        body.float_box_vec.push(float_box);

        body.float_box_index_map
            .insert(body_label.clone(), body.float_box_vec.len() - 1);
    }

    pub fn get_float_box(body_label: BodyLabel, body: &Self) -> Option<&FloatBox> {
        let &index = body.float_box_index_map.get(&body_label)?;

        body.float_box_vec.get(index)
    }

    pub fn get_float_box_mut(body_label: BodyLabel, body: &mut Self) -> Option<&mut FloatBox> {
        let &index = body.float_box_index_map.get(&body_label)?;

        body.float_box_vec.get_mut(index)
    }

    pub fn translate(delta: Vec3, body: &mut Self) {
        for float_box in &mut body.float_box_vec {
            FloatBox::translate(delta, float_box);
        }
    }
}

impl Default for Body {
    fn default() -> Self {
        Self::new(Vec3::broadcast(CELL_RADIUS_IN_METERS))
    }
}
