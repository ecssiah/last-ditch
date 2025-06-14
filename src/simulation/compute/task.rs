use crate::simulation::compute::{self};

pub trait Task: Send {
    fn execute(self: Box<Self>) -> Box<dyn compute::Result>;
}
