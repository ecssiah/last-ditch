use crate::simulation::{physics::aabb::AABB, world::chunk};
use glam::Vec3;

pub trait DynamicObject {
    fn world_position(&self) -> Vec3;
    fn set_world_position(&mut self, x: f32, y: f32, z: f32);

    fn velocity(&self) -> Vec3;
    fn set_velocity(&mut self, x: f32, y: f32, z: f32);

    fn acceleration(&self) -> Vec3;
    fn set_acceleration(&mut self, x: f32, y: f32, z: f32);

    fn height(&self) -> f32;

    fn yaw(&self) -> f32;
    fn pitch(&self) -> f32;

    fn set_yaw(&mut self, yaw: f32);
    fn set_pitch(&mut self, pitch: f32);

    fn set_rotation(&mut self, yaw: f32, pitch: f32);

    fn set_size(&mut self, size: Vec3);

    fn aabb(&self) -> AABB;
    fn set_aabb(&mut self, aabb: AABB);

    fn chunk_id(&self) -> chunk::ID;

    fn chunk_update(&self) -> bool;
    fn set_chunk_update(&mut self, chunk_update: bool);
}
