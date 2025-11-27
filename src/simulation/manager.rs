pub mod message;
pub mod status;

pub use message::Message;

use crate::simulation::{
    constants::{SIMULATION_MAX_TICKS_PER_FRAME, SIMULATION_TICK_DURATION},
    manager::status::Status,
    state::State,
};
use std::time::{Duration, Instant};
use tokio::sync::mpsc::UnboundedReceiver;

pub struct Manager {
    pub status: Status,
    pub start_instant: Instant,
    pub next_instant: Instant,
    pub ticks_total: u32,
    pub ticks_frame: u32,
    pub message_rx: UnboundedReceiver<Message>,
}

impl Manager {
    pub fn new(message_rx: UnboundedReceiver<Message>) -> Self {
        let status = Status::Init;
        let start_instant = Instant::now();
        let next_instant = Instant::now();
        let ticks_total = 0;
        let ticks_frame = 0;

        Self {
            status,
            start_instant,
            next_instant,
            ticks_total,
            ticks_frame,
            message_rx,
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
        while let Ok(message) = manager.message_rx.try_recv() {
            Manager::handle_message(&message, state);
        }

        manager.ticks_total += 1;
        manager.ticks_frame += 1;

        manager.next_instant =
            manager.start_instant + manager.ticks_total * SIMULATION_TICK_DURATION;
    }

    fn handle_message(message: &Message, state: &mut State) {
        match message {
            Message::Interact1 => todo!(),
            Message::Interact2 => todo!(),
            Message::Move(move_data) => todo!(),
            Message::Rotate(rotate_data) => todo!(),
            Message::Jump => todo!(),
            Message::Debug => todo!(),
            Message::Start => todo!(),
            Message::Quit => todo!(),
            Message::Option1 => todo!(),
            Message::Option2 => todo!(),
            Message::Option3 => todo!(),
            Message::Option4 => todo!(),
        }
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
