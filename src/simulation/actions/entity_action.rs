use glam::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct MovementAction {
    pub direction: Vec3,
    pub rotation: Vec3,
}

#[derive(Debug)]
pub enum JumpAction {
    Start,
    End,
}

#[derive(Debug)]
pub enum EntityAction {
    Movement(MovementAction),
    Jump(JumpAction),
}
