pub mod info;
pub mod kind;
pub mod kinematic;
pub mod nation;
pub mod sense;
pub mod spatial;

pub use info::Info;
pub use kind::Kind;
pub use kinematic::Kinematic;
pub use nation::Nation;
pub use sense::Sense;
pub use spatial::Spatial;

use crate::{
    simulation::{
        constants::JUDGE_PITCH_LIMIT,
        state::{World, physics::aabb::AABB, population::entity::sense::Touch},
    }, utils::ld_math::rotor3_ext::Rotor3Ext,
};
use ultraviolet::{Rotor3, Vec3};

pub struct Entity {
    pub info: Info,
    pub spatial: Spatial,
    pub kinematic: Kinematic,
    pub sense: Sense,
}

impl Entity {
    pub fn tick(world: &World, entity: &mut Entity) {
        Info::tick(&entity.spatial, world, &mut entity.info);
    }

    pub fn set_world_position(world_position: Vec3, entity: &mut Entity) {
        entity.spatial.world_position = world_position;

        Touch::set_world_position(world_position, &mut entity.sense.touch);
    }

    pub fn set_size(size: Vec3, entity: &mut Entity) {
        entity.spatial.size = size;

        entity.sense.touch.body = AABB::new(entity.sense.touch.body.center(), size);
    }

    pub fn set_rotation(yaw: f32, pitch: f32, entity: &mut Entity) {
        entity.spatial.yaw = yaw.to_radians();
        entity.spatial.pitch = pitch.to_radians();

        entity.spatial.pitch = entity
            .spatial
            .pitch
            .clamp(-JUDGE_PITCH_LIMIT, JUDGE_PITCH_LIMIT);

        entity.spatial.rotor = Rotor3::from_euler_angles(0.0, 0.0, -entity.spatial.yaw)
            * Rotor3::from_euler_angles(0.0, entity.spatial.pitch, 0.0);

        let velocity_xy = Vec3::new(
            entity.kinematic.velocity.x,
            entity.kinematic.velocity.z,
            0.0,
        );

        let speed = velocity_xy.mag_sq();

        if speed > 1e-12 {
            let new_velocity_xy = Rotor3Ext::forward(entity.spatial.rotor);

            entity.kinematic.velocity.x = new_velocity_xy.x;
            entity.kinematic.velocity.y = new_velocity_xy.y;
        }
    }
}
