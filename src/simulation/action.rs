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
    SetLinearSpeed(f32),
    SetStrafeSpeed(f32),
    SetAngularSpeed(f32),
}
