use std::ops::{Add, AddAssign};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tick(u64);

impl Tick {
    pub const ZERO: Tick = Tick(0);
}

impl Add<u64> for Tick {
    type Output = Tick;

    fn add(self, rhs: u64) -> Tick {
        Tick(self.0 + rhs)
    }
}

impl AddAssign<u64> for Tick {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs;
    }
}

impl From<Tick> for u32 {
    fn from(tick: Tick) -> Self {
        tick.0 as u32
    }
}

impl From<Tick> for u64 {
    fn from(tick: Tick) -> Self {
        tick.0
    }
}
