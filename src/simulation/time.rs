pub mod tick;

pub use tick::Tick;

use crate::simulation::FIXED_DT;
use std::time::{Duration, Instant};

pub struct Time {
    pub tick: Tick,
    pub instant: Instant,
    work_time: Duration,
}

impl Time {
    pub fn new() -> Time {
        let time = Time {
            tick: Tick::ZERO,
            instant: Instant::now(),
            work_time: Duration::ZERO,
        };

        time
    }

    pub fn has_work(&self) -> bool {
        self.work_time >= FIXED_DT
    }

    pub fn calculate_work(&mut self) {
        let now = Instant::now();
        let frame_time = now.duration_since(self.instant);
        self.instant = now;

        self.work_time += frame_time;
    }

    pub fn tick(&mut self) {
        self.tick += 1;

        self.work_time -= FIXED_DT;
    }
}
