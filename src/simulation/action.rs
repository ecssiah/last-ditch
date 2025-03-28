use glam::Vec3;

#[derive(Debug)]
pub enum Action {
    World(WorldAction),
    Agent(AgentAction),
}

#[derive(Debug)]
pub enum WorldAction {
    Quit,
}

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
pub enum AgentAction {
    Movement(MovementAction),
    Jump(JumpAction),
}
