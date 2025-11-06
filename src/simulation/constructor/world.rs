use crate::simulation::{self, state::World};

pub mod empty;
pub mod graph;
pub mod main;
pub mod test;

pub fn construct(simulation_kind: simulation::Kind, world: &mut World) {
    match simulation_kind {
        crate::simulation::Kind::Placeholder => {},
        crate::simulation::Kind::Empty => empty::construct(world),
        crate::simulation::Kind::Main => main::construct(world),
        crate::simulation::Kind::Test => test::construct(world),
        crate::simulation::Kind::Graph => graph::construct(world),
    }
}