use crate::simulation::state::world::graph::{self, Edge};
use std::fmt;

#[derive(Clone)]
pub struct Step {
    pub edge: graph::Edge,
    pub pending: bool,
    pub edge_index: usize,
    pub edge_vec: Option<Vec<Edge>>,
}

impl Step {
    pub fn new(edge: Edge) -> Self {
        Self {
            edge,
            pending: false,
            edge_index: 0,
            edge_vec: None,
        }
    }
}

impl fmt::Debug for Step {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "STEP")?;
        writeln!(formatter, "  Pending: {:?}", self.pending)?;
        writeln!(formatter, "  Edge: {:?}", self.edge)?;
        writeln!(formatter, "")
    }
}

pub struct Path {
    pub step_index: usize,
    pub step_vec: Vec<Step>,
}

impl Path {
    pub fn new() -> Self {
        Self {
            step_index: 0,
            step_vec: Vec::new(),
        }
    }
}

impl fmt::Debug for Path {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "Path:")?;

        if self.step_vec.is_empty() {
            writeln!(formatter, "  EMPTY")?;
        } else {
            for step_index in 0..self.step_vec.len() {
                writeln!(formatter, "{:?}", step_index)?;
                writeln!(formatter, "{:?}", self.step_vec[step_index])?;
            }
        }

        write!(formatter, "")
    }
}
