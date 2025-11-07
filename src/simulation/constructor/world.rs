use crate::simulation::{self, state::World};

pub mod empty;
pub mod graph;
pub mod main;
pub mod test;

pub fn run(simulation_kind: simulation::Kind, world: &mut World) {
    match simulation_kind {
        crate::simulation::Kind::Placeholder => {},
        crate::simulation::Kind::Empty => empty::run(world),
        crate::simulation::Kind::Main => main::run(world),
        crate::simulation::Kind::Test => test::run(world),
        crate::simulation::Kind::Graph => graph::run(world),
    }
}