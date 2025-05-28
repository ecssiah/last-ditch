use crate::simulation::{physics::aabb::AABB, world::chunk};
use glam::Vec3;

pub trait DynamicObject {
    fn position(&self) -> Vec3;
    fn set_position(&mut self, x: f32, y: f32, z: f32);

    fn velocity(&self) -> Vec3;
    fn set_velocity(&mut self, x: f32, y: f32, z: f32);

    fn acceleration(&self) -> Vec3;
    fn set_acceleration(&mut self, x: f32, y: f32, z: f32);

    fn size(&self) -> Vec3;

    fn aabb(&self) -> AABB;
    fn set_aabb(&mut self, aabb: AABB);

    fn chunk_id(&self) -> chunk::ID;

    fn chunk_update(&self) -> bool;
    fn set_chunk_update(&mut self, chunk_update: bool);
}
