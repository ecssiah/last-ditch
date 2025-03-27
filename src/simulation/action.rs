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
pub struct MovementActions {
    pub direction: Vec3,
    pub rotation: Vec3,
    pub is_jumping: bool,
}

#[derive(Debug)]
pub enum AgentAction {
    Movement(MovementActions),
}
