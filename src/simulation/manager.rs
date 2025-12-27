pub mod message;
pub mod status;
pub mod timestep;
pub mod viewer;

pub use message::Message;
pub use timestep::Timestep;
pub use viewer::Viewer;

use crate::simulation::{
    constants::*,
    manager::{status::Status, viewer::view::View},
    state::{
        action::{
            act::{self, JumpData, PlaceBlockData, RemoveBlockData},
            Act,
        },
        population::motion::{self},
        work::{
            construct_task::{generate_data::GenerateData, ConstructTask},
            construct_worker::ConstructWorker,
        },
        world::block::Kind,
        State,
    },
};
use std::time::{Duration, Instant};
use ultraviolet::Vec3;

pub struct Manager {
    pub status: Status,
    pub timestep: Timestep,
    pub viewer: Viewer,
    pub message_rx: crossbeam::channel::Receiver<Message>,
}

impl Manager {
    pub fn new(
        message_rx: crossbeam::channel::Receiver<Message>,
        view_input: triple_buffer::Input<View>,
    ) -> Self {
        let status = Status::Run;
        let timestep = Timestep::new();
        let viewer = Viewer::new(view_input);

        Self {
            status,
            timestep,
            viewer,
            message_rx,
        }
    }

    pub fn start(manager: &mut Self) {
        manager.timestep.ticks_frame = 0;
    }

    pub fn has_work(manager: &Self) -> bool {
        Instant::now() >= manager.timestep.next_instant
            && manager.timestep.ticks_frame < SIMULATION_MAX_TICKS_PER_FRAME
    }

    pub fn tick(state: &mut State, manager: &mut Self) -> bool {
        Self::handle_messages(state, manager);
        Self::update_timestep(manager);

        Viewer::tick(state, manager);

        match manager.status {
            Status::Run => State::tick(state),
            Status::Done => return false,
        }

        true
    }

    fn handle_messages(state: &mut State, manager: &mut Self) {
        while let Ok(message) = manager.message_rx.try_recv() {
            Self::handle_message(&message, state, manager);
        }
    }

    fn handle_message(message: &Message, state: &mut State, manager: &mut Self) {
        match message {
            Message::Interact1 => Self::handle_interact1(state),
            Message::Interact2 => Self::handle_interact2(state),
            Message::RotatationInput(rotate_data) => {
                Self::handle_rotation_input_message(rotate_data, state)
            }
            Message::MovementInput(move_data) => {
                Self::handle_movement_input_message(move_data, state)
            }
            Message::JumpInput => Self::handle_jump_input_message(state),
            Message::SetSeed(seed_data) => Self::handle_set_seed_message(seed_data, state),
            Message::Generate => Self::handle_generate_message(state),
            Message::Quit => Self::handle_quit_message(state, manager),
            Message::Debug => Self::handle_debug_message(state),
            Message::Option1 => Self::handle_option1_message(state),
            Message::Option2 => Self::handle_option2_message(state),
            Message::Option3 => Self::handle_option3_message(state),
            Message::Option4 => Self::handle_option4_message(state),
        }
    }

    fn handle_interact1(state: &mut State) {
        let place_block_data = PlaceBlockData {
            person_id: ID_JUDGE_1,
        };

        state
            .action
            .act_deque
            .push_back(Act::PlaceBlock(place_block_data));
    }

    fn handle_interact2(state: &mut State) {
        let remove_block_data = RemoveBlockData {
            person_id: ID_JUDGE_1,
        };

        state
            .action
            .act_deque
            .push_back(Act::RemoveBlock(remove_block_data));
    }

    fn handle_rotation_input_message(
        rotation_input_data: &message::RotationInputData,
        state: &mut State,
    ) {
        let rotate_data = act::RotateData {
            person_id: ID_JUDGE_1,
            rotation_angles: Vec3::new(
                rotation_input_data.input_x,
                rotation_input_data.input_y,
                rotation_input_data.input_z,
            ),
        };

        state.action.act_deque.push_back(Act::Rotate(rotate_data));
    }

    fn handle_movement_input_message(
        movement_input_data: &message::MovementInputData,
        state: &mut State,
    ) {
        let move_direction = Vec3::new(
            movement_input_data.input_x,
            movement_input_data.input_y,
            movement_input_data.input_z,
        )
        .normalized();

        let move_data = act::MoveData {
            person_id: ID_JUDGE_1,
            move_direction,
        };

        state.action.act_deque.push_back(Act::Move(move_data));
    }

    fn handle_jump_input_message(state: &mut State) {
        let jump_data = JumpData {
            person_id: ID_JUDGE_1,
        };

        state.action.act_deque.push_back(Act::Jump(jump_data));
    }

    fn handle_set_seed_message(seed_data: &message::SeedData, state: &mut State) {
        State::seed(seed_data.seed, state);
    }

    fn handle_generate_message(state: &mut State) {
        let generate_data = GenerateData::new();
        let construct_task = ConstructTask::Generate(generate_data);

        ConstructWorker::enqueue(construct_task, &mut state.work.construct_worker.task_deque);

        state.active = true;
    }

    fn handle_quit_message(_state: &mut State, manager: &mut Self) {
        // TODO: Save Simulation State!

        manager.status = Status::Done;
    }

    fn handle_debug_message(state: &mut State) {
        if let Some(judge) = state.population.person_map.get_mut(&ID_JUDGE_1) {
            match judge.motion.mode {
                motion::Mode::Ground | motion::Mode::Climb => {
                    judge.motion.mode = motion::Mode::Air;
                }
                motion::Mode::Air => {
                    judge.motion.mode = motion::Mode::Ground;
                }
            }
        }
    }

    fn handle_option1_message(state: &mut State) {
        if let Some(judge) = state.population.person_map.get_mut(&ID_JUDGE_1) {
            judge.selected_block_kind = Kind::previous_block_kind(&judge.selected_block_kind);
        }
    }

    fn handle_option2_message(state: &mut State) {
        if let Some(judge) = state.population.person_map.get_mut(&ID_JUDGE_1) {
            judge.selected_block_kind = Kind::next_block_kind(&judge.selected_block_kind);
        }
    }

    fn handle_option3_message(_state: &mut State) {
        tracing::info!("Option 3 Message");
    }

    fn handle_option4_message(_state: &mut State) {
        tracing::info!("Option 4 Message");
    }

    pub fn update_timestep(manager: &mut Self) {
        manager.timestep.ticks_total += 1;
        manager.timestep.ticks_frame += 1;

        manager.timestep.next_instant = manager.timestep.start_instant
            + manager.timestep.ticks_total * SIMULATION_TICK_DURATION;
    }

    pub fn fix_timestep(manager: &mut Self) {
        let current_instant = Instant::now();

        if current_instant < manager.timestep.next_instant {
            let remaining_duration = manager.timestep.next_instant - current_instant;

            if remaining_duration > Duration::from_millis(2) {
                std::thread::sleep(remaining_duration - Duration::from_millis(1));
            }

            while Instant::now() < manager.timestep.next_instant {
                std::hint::spin_loop();
            }
        }
    }
}
