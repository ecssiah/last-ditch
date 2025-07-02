use crate::simulation::consts::*;
use std::time::{Duration, Instant};

struct Instants {
    start: Instant,
    next: Instant,
}

struct Ticks {
    total: u32,
    frame: u32,
}

pub struct Timing {
    ticks: Ticks,
    instants: Instants,
}

impl Timing {
    pub fn new() -> Self {
        let instants = Instants {
            start: Instant::now(),
            next: Instant::now(),
        };

        let ticks = Ticks { total: 0, frame: 0 };

        Self { instants, ticks }
    }

    pub fn init(&mut self) {
        self.instants.start = Instant::now();
        self.instants.next = self.instants.start;
    }

    pub fn has_work(&self) -> bool {
        Instant::now() >= self.instants.next && self.ticks.frame < SIMULATION_MAX_TICKS_PER_FRAME
    }

    pub fn start_frame(&mut self) {
        self.ticks.frame = 0;
    }

    pub fn update_frame(&mut self) {
        self.ticks.total += 1;
        self.ticks.frame += 1;

        self.instants.next = self.instants.start + self.ticks.total * SIMULATION_TICK_DURATION;
    }

    pub fn fix_timestep(&mut self) {
        let current_instant = Instant::now();

        if current_instant < self.instants.next {
            let remaining_duration = self.instants.next - current_instant;

            if remaining_duration > Duration::from_millis(2) {
                std::thread::sleep(remaining_duration - Duration::from_millis(1));
            }

            while Instant::now() < self.instants.next {
                std::hint::spin_loop();
            }
        }
    }
}
