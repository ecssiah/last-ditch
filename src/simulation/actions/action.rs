use crate::simulation::actions::{EntityAction, WorldAction};

#[derive(Debug)]
pub enum Action {
    World(WorldAction),
    Agent(EntityAction),
}
