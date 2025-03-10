#[derive(Debug)]
pub enum Action {
    World(WorldAction),
    Agent(AgentAction),
}

#[derive(Debug)]
pub enum WorldAction {
    Quit,
}

#[derive(Debug, Copy, Clone)]
pub struct MoveActions {
    pub forward: f32,
    pub backward: f32,
    pub left: f32,
    pub right: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct RotateActions {
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Debug)]
pub enum AgentAction {
    Move(MoveActions),
    Rotate(RotateActions),
}
