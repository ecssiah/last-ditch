use crate::simulation::state::world::graph::{self, Edge};
use glam::IVec3;
use std::fmt;

#[derive(Clone)]
pub struct Step {
    pub edge: graph::Edge,
    pub index: usize,
    pub position_vec: Option<Vec<IVec3>>,
}

impl Step {
    pub fn new(edge: Edge) -> Self {
        Self {
            edge,
            index: 0,
            position_vec: None,
        }
    }
}

pub struct Path {
    pub current_step: usize,
    pub step_vec: Vec<Step>,
}

impl Path {
    pub fn new() -> Self {
        Self {
            current_step: 0,
            step_vec: Vec::new(),
        }
    }
}

impl fmt::Debug for Path {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "Path")?;
        write!(formatter, "")
    }
}
