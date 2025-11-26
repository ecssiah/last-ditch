use crate::simulation::constants::*;
use std::time::{Duration, Instant};

pub struct Timestep {
    pub start_instant: Instant,
    pub next_instant: Instant,
    pub ticks_total: u32,
    pub ticks_frame: u32,
}

impl Timestep {
    pub fn new() -> Self {
        Self {
            start_instant: Instant::now(),
            next_instant: Instant::now(),
            ticks_total: 0,
            ticks_frame: 0,
        }
    }

    pub fn init(timestep: &mut Timestep) {
        timestep.start_instant = Instant::now();
        timestep.next_instant = timestep.start_instant;
    }

    pub fn has_work(timestep: &Timestep) -> bool {
        Instant::now() >= timestep.next_instant
            && timestep.ticks_frame < SIMULATION_MAX_TICKS_PER_FRAME
    }

    pub fn start(timestep: &mut Timestep) {
        timestep.ticks_frame = 0;
    }

    pub fn tick(timestep: &mut Timestep) {
        timestep.ticks_total += 1;
        timestep.ticks_frame += 1;

        timestep.next_instant =
            timestep.start_instant + timestep.ticks_total * SIMULATION_TICK_DURATION;
    }

    pub fn fix(timestep: &mut Timestep) {
        let current_instant = Instant::now();

        if current_instant < timestep.next_instant {
            let remaining_duration = timestep.next_instant - current_instant;

            if remaining_duration > Duration::from_millis(2) {
                std::thread::sleep(remaining_duration - Duration::from_millis(1));
            }

            while Instant::now() < timestep.next_instant {
                std::hint::spin_loop();
            }
        }
    }
}
