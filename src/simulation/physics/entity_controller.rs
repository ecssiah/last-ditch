use crate::simulation::population::entity;
use rapier3d::{
    control::KinematicCharacterController,
    prelude::{RigidBodyHandle, SharedShape},
};

pub struct EntityController {
    pub entity_id: entity::ID,
    pub shape: SharedShape,
    pub rigid_body_handle: RigidBodyHandle,
    pub character_controller: KinematicCharacterController,
}
