use glam::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct MovementAction {
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
    Movement(MovementAction),
    Jump(JumpAction),
}
