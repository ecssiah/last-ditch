use crate::simulation::dispatch::{EntityAction, WorldAction};

#[derive(Debug)]
pub enum Action {
    World(WorldAction),
    Agent(EntityAction),
}
