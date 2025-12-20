use ultraviolet::Vec3;

use crate::{
    simulation::{
        constants::CELL_SIZE_IN_METERS,
        state::physics::collider::{self, Collider},
    },
    utils::ldmath::FloatBox,
};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Body {
    pub local_position: Vec3,
    pub world_position: Vec3,
    pub collider_vec: Vec<Collider>,
    pub collider_index_map: HashMap<collider::Label, usize>,
}

impl Body {
    pub fn new(size: Vec3) -> Self {
        let local_position = Vec3::zero();
        let world_position = Vec3::zero();
        let collider_vec = vec![Collider::new(Vec3::zero(), size)];
        let collider_index_map = HashMap::from([(collider::Label::Core, collider_vec.len() - 1)]);

        Self {
            local_position,
            world_position,
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

    pub fn get_world_position(body: &Self) -> Vec3 {
        body.world_position
    }

    pub fn set_world_position(parent_world_position: Vec3, body: &mut Self) {
        body.world_position = parent_world_position + body.local_position;

        for collider in &mut body.collider_vec {
            Collider::set_world_position(body.world_position, collider);
        }
    }

    pub fn get_size(body: &Self) -> Vec3 {
        let core = Self::get_collider(collider::Label::Core, body).expect("Body has no core");

        Collider::get_size(core)
    }

    pub fn set_size(size: Vec3, body: &mut Self) {
        let core = Self::get_collider_mut(collider::Label::Core, body).expect("Body has no core");

        Collider::set_size(size, core);
    }

    pub fn get_float_box(body: &Self) -> FloatBox {
        let core = Self::get_collider(collider::Label::Core, body).expect("Body has no core");

        core.float_box.clone()
    }

    pub fn set_float_box(float_box: FloatBox, body: &mut Self) {
        let core = Self::get_collider_mut(collider::Label::Core, body).expect("Body has no core");

        core.float_box = float_box;
    }
}

impl Default for Body {
    fn default() -> Self {
        Self::new(Vec3::broadcast(CELL_SIZE_IN_METERS))
    }
}
