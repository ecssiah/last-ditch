use crate::simulation::physics::aabb::AABB;

pub trait Dynamic {
    fn aabb(&self) -> AABB;
    fn aabb_mut(&mut self) -> &mut AABB;
}