use cgmath::Vector3;

#[derive(Debug)]
pub enum Action {
    World(WorldAction),
    Entity(EntityAction),
}

#[derive(Debug)]
pub enum WorldAction {
    Quit,
}

#[derive(Debug)]
pub enum EntityAction {
    Move(Vector3<f32>),
    Rotate(Vector3<f32>),
}
