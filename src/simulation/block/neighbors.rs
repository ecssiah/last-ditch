use crate::simulation::block;
use bitflags::bitflags;
use serde::Deserialize;

bitflags! {
    #[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq)]
    pub struct Neighbors: u32 {
        const NONE = 0;
    }
}

impl Neighbors {
    pub fn is_solid(&self, direction: block::Direction) -> bool {
        self.bits() & direction.bits() != 0
    }

    pub fn set_solid(&mut self, direction: block::Direction, solid: bool) {
        if solid {
            self.insert(Neighbors::from_bits_retain(self.bits() | direction.bits()));
        } else {
            self.remove(Neighbors::from_bits_retain(self.bits() & !direction.bits()));
        }
    }
}
