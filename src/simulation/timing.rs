use crate::simulation::consts::*;
use std::time::{Duration, Instant};

pub struct Timing {
    start_instant: Instant,
    next_instant: Instant,
    ticks_total: u32,
    ticks_frame: u32,
}

impl Timing {
    pub fn new() -> Self {
        Self {
            start_instant: Instant::now(),
            next_instant: Instant::now(),
            ticks_total: 0,
            ticks_frame: 0,
        }
    }

    pub fn init(&mut self) {
        self.start_instant = Instant::now();
        self.next_instant = self.start_instant;
    }

    pub fn has_work(&self) -> bool {
        Instant::now() >= self.next_instant && self.ticks_frame < SIMULATION_MAX_TICKS_PER_FRAME
    }

    pub fn start_frame(&mut self) {
        self.ticks_frame = 0;
    }

    pub fn update_frame(&mut self) {
        self.ticks_total += 1;
        self.ticks_frame += 1;

        self.next_instant = self.start_instant + self.ticks_total * SIMULATION_TICK_DURATION;
    }

    pub fn fix_timestep(&mut self) {
        let current_instant = Instant::now();

        if current_instant < self.next_instant {
            let remaining_duration = self.next_instant - current_instant;

            if remaining_duration > Duration::from_millis(2) {
                std::thread::sleep(remaining_duration - Duration::from_millis(1));
            }

            while Instant::now() < self.next_instant {
                std::hint::spin_loop();
            }
        }
    }
}
