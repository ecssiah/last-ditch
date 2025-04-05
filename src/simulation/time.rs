use crate::simulation::{FIXED_DT, FIXED_TICK_RATE};
use std::time::{Duration, Instant};

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
    pub work_time: Duration,
    pub previous_instant: Instant,
}

impl Time {
    pub fn new() -> Time {
        let time = Time {
            clock: Duration::ZERO,
            tick: Tick::ZERO,
            work_time: Duration::ZERO,
            previous_instant: Instant::now(),
        };

        time
    }

    pub fn calculate_work_time(&mut self) {
        let now = Instant::now();
        let frame_time = now.duration_since(self.previous_instant);
        self.previous_instant = now;

        self.work_time += frame_time;
    }

    pub fn tick(&mut self) {
        self.clock += FIXED_DT;
        self.tick.advance();
    }

    pub fn has_work_time(&self) -> bool {
        self.work_time >= FIXED_DT
    }

    pub fn use_work_time(&mut self) {
        self.work_time -= FIXED_DT;
    }
}
