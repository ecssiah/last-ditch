use crate::simulation::population::judge;
use rapier3d::prelude::{ColliderHandle, RigidBodyHandle};

pub struct JudgeController {
    pub judge_id: judge::ID,
    pub rigid_body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
}
