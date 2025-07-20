use crate::simulation::state::world::graph::Step;
use std::fmt;

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
