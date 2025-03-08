#[derive(Debug)]
pub enum Action {
    World(WorldAction),
    Entity(EntityAction),
}

#[derive(Debug)]
pub enum WorldAction {
    Quit,
}

#[derive(Debug, Copy, Clone)]
pub struct InputActions {
    pub forward: f32,
    pub back: f32,
    pub left: f32,
    pub right: f32,
    pub turn_left: f32,
    pub turn_right: f32,
}

#[derive(Debug)]
pub enum EntityAction {
    Input(InputActions)
}
