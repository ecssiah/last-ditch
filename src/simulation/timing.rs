use crate::simulation::constants::*;
use std::time::{Duration, Instant};

pub struct Timing {
    pub start_instant: Instant,
    pub next_instant: Instant,
    pub ticks_total: u32,
    pub ticks_frame: u32,
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

    pub fn init(timing: &mut Timing) {
        timing.start_instant = Instant::now();
        timing.next_instant = timing.start_instant;
    }

    pub fn has_work(timing: &Timing) -> bool {
        Instant::now() >= timing.next_instant && timing.ticks_frame < SIMULATION_MAX_TICKS_PER_FRAME
    }

    pub fn start_frame(timing: &mut Timing) {
        timing.ticks_frame = 0;
    }

    pub fn update_frame(timing: &mut Timing) {
        timing.ticks_total += 1;
        timing.ticks_frame += 1;

        timing.next_instant = timing.start_instant + timing.ticks_total * SIMULATION_TICK_DURATION;
    }

    pub fn fix_timestep(timing: &mut Timing) {
        let current_instant = Instant::now();

        if current_instant < timing.next_instant {
            let remaining_duration = timing.next_instant - current_instant;

            if remaining_duration > Duration::from_millis(2) {
                std::thread::sleep(remaining_duration - Duration::from_millis(1));
            }

            while Instant::now() < timing.next_instant {
                std::hint::spin_loop();
            }
        }
    }
}
