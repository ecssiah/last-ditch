#[derive(Debug)]
pub enum Action {
    World(WorldAction),
}

#[derive(Debug)]
pub enum WorldAction {
    Quit,
}
