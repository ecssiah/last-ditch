use std::time::Duration;
use crate::simulation::FIXED_TICK_RATE;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tick(pub u64);

impl Tick {
    pub const ZERO: Tick = Tick(0);

    pub fn advance(&mut self) {
        self.0 += 1;
    }

    pub fn next(self) -> Tick {
        Tick(self.0 + 1)
    }

    pub fn prev(self) -> Tick {
        Tick(self.0.saturating_sub(1))
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }

    pub fn as_duration(self) -> Duration {
        Duration::from_nanos((1_000_000_000 / FIXED_TICK_RATE as u64) * self.0)
    }

    pub fn from_duration(d: Duration) -> Tick {
        let nanos_per_tick = 1_000_000_000 / FIXED_TICK_RATE as u64;
        Tick(d.as_nanos() as u64 / nanos_per_tick)
    }
}

pub struct Time {
    pub clock: Duration,
    pub tick: Tick,
}