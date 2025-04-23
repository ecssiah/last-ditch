use glam::IVec3;

#[derive(Clone, Debug)]
pub enum Goal {
    Idle,
    Wander,
    Seek(IVec3),
}
