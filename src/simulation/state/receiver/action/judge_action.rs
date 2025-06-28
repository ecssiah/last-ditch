use glam::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct MovementData {
    pub direction: Vec3,
    pub rotation: Vec3,
}

#[derive(Clone, Copy, Debug)]
pub enum JumpAction {
    Start,
    End,
}

#[derive(Clone, Copy, Debug)]
pub enum JudgeAction {
    Movement(MovementData),
    Jump(JumpAction),
}
