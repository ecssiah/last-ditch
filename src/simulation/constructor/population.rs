use crate::simulation::{
    self,
    state::{Population, World},
};

pub mod empty;
pub mod graph;
pub mod main;
pub mod test;

pub fn run(simulation_kind: simulation::Kind, world: &World, entity: &mut Population) {
    match simulation_kind {
        crate::simulation::Kind::Placeholder => {}
        crate::simulation::Kind::Empty => empty::run(entity),
        crate::simulation::Kind::Main => main::run(world, entity),
        crate::simulation::Kind::Test => test::run(entity),
        crate::simulation::Kind::Graph => graph::run(entity),
    }
}
