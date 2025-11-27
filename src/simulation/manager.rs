use crate::simulation::{
    constants::{SIMULATION_MAX_TICKS_PER_FRAME, SIMULATION_TICK_DURATION},
    state::{action::Act, State},
};
use std::time::{Duration, Instant};
use tokio::sync::mpsc::UnboundedReceiver;

pub struct Manager {
    pub active: bool,
    pub loading: bool,
    pub start_instant: Instant,
    pub next_instant: Instant,
    pub ticks_total: u32,
    pub ticks_frame: u32,
    pub act_rx: UnboundedReceiver<Act>,
}

impl Manager {
    pub fn new(act_rx: UnboundedReceiver<Act>) -> Self {
        let active = true;
        let loading = false;
        let start_instant = Instant::now();
        let next_instant = Instant::now();
        let ticks_total = 0;
        let ticks_frame = 0;

        Self {
            active,
            loading,
            start_instant,
            next_instant,
            ticks_total,
            ticks_frame,
            act_rx,
        }
    }

    pub fn init(manager: &mut Manager) {
        manager.start_instant = Instant::now();
        manager.next_instant = manager.start_instant;
    }

    pub fn start(manager: &mut Manager) {
        manager.ticks_frame = 0;
    }

    pub fn has_work(manager: &Manager) -> bool {
        Instant::now() >= manager.next_instant
            && manager.ticks_frame < SIMULATION_MAX_TICKS_PER_FRAME
    }

    pub fn tick(state: &mut State, manager: &mut Manager) {
        while let Ok(act) = manager.act_rx.try_recv() {
            match act {
                Act::Exit => {
                    manager.active = false;
                    return;
                }
                _ => state.action.act_deque.push_back(act),
            }
        }

        manager.ticks_total += 1;
        manager.ticks_frame += 1;

        manager.next_instant =
            manager.start_instant + manager.ticks_total * SIMULATION_TICK_DURATION;
    }

    pub fn fix_timestep(manager: &mut Manager) {
        let current_instant = Instant::now();

        if current_instant < manager.next_instant {
            let remaining_duration = manager.next_instant - current_instant;

            if remaining_duration > Duration::from_millis(2) {
                std::thread::sleep(remaining_duration - Duration::from_millis(1));
            }

            while Instant::now() < manager.next_instant {
                std::hint::spin_loop();
            }
        }
    }
}
