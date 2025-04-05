pub mod tick;

pub use tick::Tick;

use crate::simulation::FIXED_DT;
use std::time::{Duration, Instant};

pub struct Time {
    pub clock: Duration,
    pub tick: Tick,
    pub work_time: Duration,
    pub previous: Instant,
}

impl Time {
    pub fn new() -> Time {
        let time = Time {
            clock: Duration::ZERO,
            tick: Tick::ZERO,
            work_time: Duration::ZERO,
            previous: Instant::now(),
        };

        time
    }

    pub fn tick(&mut self) {
        self.clock += FIXED_DT;
        self.tick.advance();

        self.work_time -= FIXED_DT;
    }
}
