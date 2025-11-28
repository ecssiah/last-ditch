pub mod message;
pub mod status;

pub use message::Message;

use crate::simulation::{
    constants::{SIMULATION_MAX_TICKS_PER_FRAME, SIMULATION_TICK_DURATION},
    manager::status::Status,
    state::{
        action::{
            act::{self},
            Act,
        },
        State,
    },
    viewer::{View, Viewer},
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
    pub viewer: Viewer,
}

impl Manager {
    pub fn new(
        message_rx: UnboundedReceiver<Message>,
        view_input: triple_buffer::Input<View>,
    ) -> Self {
        let status = Status::Init;
        let start_instant = Instant::now();
        let next_instant = Instant::now();
        let ticks_total = 0;
        let ticks_frame = 0;
        let viewer = Viewer::new(view_input);

        Self {
            status,
            start_instant,
            next_instant,
            ticks_total,
            ticks_frame,
            message_rx,
            viewer,
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

    pub fn tick(state: &mut State, manager: &mut Manager) -> Status {
        manager.ticks_total += 1;
        manager.ticks_frame += 1;

        manager.next_instant =
            manager.start_instant + manager.ticks_total * SIMULATION_TICK_DURATION;

        while let Ok(message) = manager.message_rx.try_recv() {
            Manager::handle_message(&message, state, manager);
        }

        Viewer::tick(state, manager);

        manager.status
    }

    fn handle_message(message: &Message, state: &mut State, manager: &mut Manager) {
        match message {
            Message::Interact1 => Self::handle_interact1(state),
            Message::Interact2 => Self::handle_interact2(state),
            Message::Rotate(rotate_data) => Self::handle_rotate_message(rotate_data, state),
            Message::Move(move_data) => Self::handle_move_message(move_data, state),
            Message::Jump => Self::handle_jump_message(state),
            Message::Debug => todo!(),
            Message::Generate(generate_data) => {
                Self::handle_generate_message(generate_data, state, manager)
            }
            Message::Quit => Self::handle_quit_message(state, manager),
            Message::Option1 => todo!(),
            Message::Option2 => todo!(),
            Message::Option3 => todo!(),
            Message::Option4 => todo!(),
        }
    }

    fn handle_interact1(state: &mut State) {
        state.action.act_deque.push_back(Act::PlaceBlock);
    }

    fn handle_interact2(state: &mut State) {
        state.action.act_deque.push_back(Act::RemoveBlock);
    }

    fn handle_rotate_message(rotate_data: &message::RotateData, state: &mut State) {
        let rotate_data = act::RotateData {
            rotate_xy: rotate_data.rotate_xy,
            rotate_yz: rotate_data.rotate_yz,
            rotate_zx: rotate_data.rotate_zx,
        };

        state.action.act_deque.push_back(Act::Rotate(rotate_data));
    }

    fn handle_move_message(move_data: &message::MoveData, state: &mut State) {
        let move_data = act::MoveData {
            move_x: move_data.move_x,
            move_y: move_data.move_y,
            move_z: move_data.move_z,
        };

        state.action.act_deque.push_back(Act::Move(move_data));
    }

    fn handle_jump_message(state: &mut State) {
        state.action.act_deque.push_back(Act::Jump);
    }

    fn handle_generate_message(
        generate_data: &message::GenerateData,
        state: &mut State,
        manager: &mut Manager,
    ) {
        State::seed(generate_data.seed, state);
        State::init(state);

        manager.status = Status::Load;
    }

    fn handle_quit_message(_state: &mut State, manager: &mut Manager) {
        // TODO: Save Simulation State!

        manager.status = Status::Done;
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
