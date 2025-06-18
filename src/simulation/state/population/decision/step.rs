use glam::IVec3;

#[derive(Clone, Debug)]
pub enum Step {
    Move(IVec3),
    Wait,
}
