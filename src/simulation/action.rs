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
    pub x_axis: f32,
    pub z_axis: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct RotateActions {
    pub x_axis: f32,
    pub y_axis: f32,
}

#[derive(Debug)]
pub enum AgentAction {
    Move(MoveActions),
    Rotate(RotateActions),
}