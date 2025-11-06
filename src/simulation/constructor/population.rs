use crate::simulation::{
    self,
    state::{Population, World},
};

pub mod empty;
pub mod graph;
pub mod main;
pub mod test;

pub fn construct(simulation_kind: simulation::Kind, world: &World, population: &mut Population) {
    match simulation_kind {
        crate::simulation::Kind::Placeholder => {}
        crate::simulation::Kind::Empty => empty::construct(population),
        crate::simulation::Kind::Main => main::construct(world, population),
        crate::simulation::Kind::Test => test::construct(population),
        crate::simulation::Kind::Graph => graph::construct(population),
    }
}
