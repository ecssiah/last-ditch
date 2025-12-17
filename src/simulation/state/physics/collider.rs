pub mod info;
pub mod kind;
pub mod owner;

pub use info::Info;
pub use kind::Kind;
pub use owner::Owner;

use crate::simulation::state::physics::collider;
use ultraviolet::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Collider {
    pub relative_position: Vec3,
    pub world_position: Vec3,
    pub size: Vec3,
    pub kind: collider::Kind,
    pub owner: collider::Owner,
}

impl Collider {
    pub fn new(world_position: Vec3, size: Vec3) -> Self {
        let relative_position = Vec3::zero();
        let kind = collider::Kind::Physics;
        let owner = collider::Owner::None;

        Self {
            relative_position,
            world_position,
            size,
            kind,
            owner,
        }
    }

    pub fn default() -> Self {
        Self::new(Vec3::zero(), Vec3::one())
    }

    pub fn set_relative_position(relative_position: Vec3, collider: &mut Self) {
        collider.relative_position = relative_position;
    }

    pub fn set_world_position(world_position: Vec3, collider: &mut Self) {
        collider.world_position = world_position;
    }

    pub fn set_size(size: Vec3, collider: &mut Self) {
        collider.size = size;
    }

    // pub fn overlaps(&self, other: Self) -> bool {
    //     self.min.x < other.max.x
    //         && self.max.x > other.min.x
    //         && self.min.y < other.max.y
    //         && self.max.y > other.min.y
    //         && self.min.z < other.max.z
    //         && self.max.z > other.min.z
    // }

    // pub fn overlap_axis(&self, axis_index: usize, cell_aabb: Self) -> f32 {
    //     let min = self.min[axis_index];
    //     let max = self.max[axis_index];

    //     let cell_min = cell_aabb.min[axis_index];
    //     let cell_max = cell_aabb.max[axis_index];

    //     if max > cell_min && min < cell_max {
    //         let offset_positive = cell_max - min;
    //         let offset_negative = max - cell_min;

    //         let center = (min + max) * 0.5;
    //         let cell_center = (cell_min + cell_max) * 0.5;

    //         if center < cell_center {
    //             offset_positive
    //         } else {
    //             -offset_negative
    //         }
    //     } else {
    //         0.0
    //     }
    // }
}
