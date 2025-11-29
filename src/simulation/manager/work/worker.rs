use crate::simulation::state::State;

pub trait Worker: Send {
    fn active(&self) -> bool;
    fn cost(&self) -> u32;
    fn work(&mut self, state: &mut State);
}
