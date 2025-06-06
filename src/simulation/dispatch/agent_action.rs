use glam::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct MovementAction {
    pub direction: Vec3,
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Debug)]
pub enum JumpAction {
    Start,
    End,
}

#[derive(Debug)]
pub enum AgentAction {
    Movement(MovementAction),
    Jump(JumpAction),
}
