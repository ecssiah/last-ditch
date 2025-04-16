use crate::simulation::population::entity;
use rapier3d::prelude::{ColliderHandle, RigidBodyHandle};

pub struct EntityController {
    pub entity_id: entity::ID,
    pub rigid_body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
}
