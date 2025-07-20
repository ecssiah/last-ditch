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
