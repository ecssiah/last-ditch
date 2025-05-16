use crate::simulation::{physics::aabb::AABB, world::chunk};
use glam::Vec3;

pub trait DynamicObject {
    fn position(&self) -> Vec3;
    fn set_position(&mut self, position: Vec3);

    fn velocity(&self) -> Vec3;
    fn set_velocity(&mut self, velocity: Vec3);

    fn size(&self) -> Vec3;

    fn aabb(&self) -> AABB;
    fn set_aabb(&mut self, aabb: AABB);

    fn chunk_id(&self) -> chunk::ID;

    fn chunk_update(&self) -> bool;
    fn set_chunk_update(&mut self, chunk_update: bool);
}
