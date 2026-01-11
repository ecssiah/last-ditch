pub mod message;
pub mod scheduler;
pub mod supervisor_status;
pub mod viewer;

pub use message::Message;
pub use viewer::Viewer;

use crate::{
    interface::constants::OVERSEER_MESSAGE_LIMIT,
    simulation::{
        constants::*,
        state::{
            action::act::{self, Act, JumpData, PlaceBlockData, RemoveBlockData},
            population::{
                motion::{self},
                person::person_id::PersonID,
            },
            work::{
                construct_task::{generate_data::GenerateData, ConstructTask},
                construct_worker::ConstructWorker,
            },
            world::block::block_kind::BlockKind,
            State,
        },
        supervisor::{
            scheduler::Scheduler, supervisor_status::SupervisorStatus, viewer::view::View,
        },
    },
};
use std::time::{Duration, Instant};
use tracing::instrument;
use ultraviolet::Vec3;

pub struct Supervisor {
    pub supervisor_status: SupervisorStatus,
    pub tick: u32,
    pub frame_ticks: u32,
    pub instant_start: Instant,
    pub instant_next: Instant,
    pub message_limit: usize,
    pub message_rx: crossbeam::channel::Receiver<Message>,
    pub viewer: Viewer,
    pub scheduler: Scheduler,
}

impl Supervisor {
    pub fn new(
        message_rx: crossbeam::channel::Receiver<Message>,
        view_input: triple_buffer::Input<View>,
    ) -> Self {
        let supervisor_status = SupervisorStatus::Start;

        let tick = 0;
        let frame_ticks = 0;
        let instant_start = Instant::now();
        let instant_next = Instant::now();

        let message_limit = OVERSEER_MESSAGE_LIMIT;

        let viewer = Viewer::new(view_input);
        let scheduler = Scheduler::new();

        Self {
            supervisor_status,
            tick,
            frame_ticks,
            instant_start,
            instant_next,
            message_limit,
            message_rx,
            viewer,
            scheduler,
        }
    }

    pub fn start_timestep(supervisor: &mut Self) {
        supervisor.frame_ticks = 0;
    }

    pub fn has_work(supervisor: &Self) -> bool {
        Instant::now() >= supervisor.instant_next
            && supervisor.frame_ticks < SIMULATION_MAX_TICKS_PER_FRAME
    }

    pub fn update_timestep(supervisor: &mut Self) {
        supervisor.tick += 1;
        supervisor.frame_ticks += 1;

        supervisor.instant_next =
            supervisor.instant_start + supervisor.tick * SIMULATION_TICK_DURATION;
    }

    pub fn fix_timestep(supervisor: &mut Self) {
        let current_instant = Instant::now();

        if current_instant < supervisor.instant_next {
            let remaining_duration = supervisor.instant_next - current_instant;

            if remaining_duration > Duration::from_millis(2) {
                std::thread::sleep(remaining_duration - Duration::from_millis(1));
            }

            while Instant::now() < supervisor.instant_next {
                std::hint::spin_loop();
            }
        }
    }

    #[instrument(skip_all)]
    pub fn tick(state: &mut State, supervisor: &mut Self) -> bool {
        Self::update_timestep(supervisor);
        Self::receive_messages(state, supervisor);

        Viewer::tick(state, supervisor);

        match supervisor.supervisor_status {
            SupervisorStatus::Start => true,
            SupervisorStatus::Run => State::tick(state),
            SupervisorStatus::Done => false,
        }
    }

    fn receive_messages(state: &mut State, supervisor: &mut Self) {
        let mut message_limit = supervisor.message_limit;

        while let Ok(message) = supervisor.message_rx.try_recv() {
            message_limit -= 1;

            if message_limit > 0 {
                match supervisor.supervisor_status {
                    SupervisorStatus::Start => {
                        Self::handle_start_message(&message, state, supervisor)
                    }
                    SupervisorStatus::Run => Self::handle_run_message(&message, state, supervisor),
                    SupervisorStatus::Done => {
                        Self::handle_done_message(&message, state, supervisor)
                    }
                }
            }
        }
    }

    fn handle_start_message(message: &Message, state: &mut State, supervisor: &mut Self) {
        match message {
            Message::SetSeed(seed_data) => Self::handle_set_seed_message(seed_data, state),
            Message::Generate => Self::handle_generate_message(state),
            Message::Quit => Self::handle_quit_message(state, supervisor),
            _ => (),
        }
    }

    fn handle_run_message(message: &Message, state: &mut State, supervisor: &mut Self) {
        match message {
            Message::Interact1 => Self::handle_interact1(state),
            Message::Interact2 => Self::handle_interact2(state),
            Message::RotateInput(rotate_data) => {
                Self::handle_rotate_input_message(rotate_data, state)
            }
            Message::MoveInput(move_data) => Self::handle_move_input_message(move_data, state),
            Message::JumpInput => Self::handle_jump_input_message(state),
            Message::SetSeed(seed_data) => Self::handle_set_seed_message(seed_data, state),
            Message::Generate => Self::handle_generate_message(state),
            Message::Quit => Self::handle_quit_message(state, supervisor),
            Message::Debug => Self::handle_debug_message(state),
            Message::Option1 => Self::handle_option1_message(state),
            Message::Option2 => Self::handle_option2_message(state),
            Message::Option3 => Self::handle_option3_message(state),
            Message::Option4 => Self::handle_option4_message(state),
        }
    }

    fn handle_done_message(_message: &Message, _state: &mut State, _supervisor: &mut Self) {
        
    }

    fn handle_interact1(state: &mut State) {
        let place_block_data = PlaceBlockData {
            person_id: PersonID::JUDGE_ID_1,
        };

        state
            .action
            .act_deque
            .push_back(Act::PlaceBlock(place_block_data));
    }

    fn handle_interact2(state: &mut State) {
        let remove_block_data = RemoveBlockData {
            person_id: PersonID::JUDGE_ID_1,
        };

        state
            .action
            .act_deque
            .push_back(Act::RemoveBlock(remove_block_data));
    }

    fn handle_rotate_input_message(
        rotate_input_data: &message::RotateInputData,
        state: &mut State,
    ) {
        let rotate_data = act::RotateData {
            person_id: PersonID::JUDGE_ID_1,
            rotation_angles: Vec3::new(
                rotate_input_data.input_x,
                rotate_input_data.input_y,
                rotate_input_data.input_z,
            ),
        };

        state.action.act_deque.push_back(Act::Rotate(rotate_data));
    }

    fn handle_move_input_message(move_input_data: &message::MoveInputData, state: &mut State) {
        let move_direction = Vec3::new(
            move_input_data.input_x,
            move_input_data.input_y,
            move_input_data.input_z,
        )
        .normalized();

        let move_data = act::MoveData {
            person_id: PersonID::JUDGE_ID_1,
            move_direction,
        };

        state.action.act_deque.push_back(Act::Move(move_data));
    }

    fn handle_jump_input_message(state: &mut State) {
        let jump_data = JumpData {
            person_id: PersonID::JUDGE_ID_1,
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

    fn handle_quit_message(_state: &mut State, supervisor: &mut Self) {
        // TODO: Save Simulation State!

        supervisor.supervisor_status = SupervisorStatus::Done;
    }

    fn handle_debug_message(state: &mut State) {
        if let Some(judge) = state.population.person_map.get_mut(&PersonID::JUDGE_ID_1) {
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
        if let Some(judge) = state.population.person_map.get_mut(&PersonID::JUDGE_ID_1) {
            judge.selected_block_kind =
                BlockKind::get_previous_block_kind(&judge.selected_block_kind);
        }
    }

    fn handle_option2_message(state: &mut State) {
        if let Some(judge) = state.population.person_map.get_mut(&PersonID::JUDGE_ID_1) {
            judge.selected_block_kind = BlockKind::get_next_block_kind(&judge.selected_block_kind);
        }
    }

    fn handle_option3_message(_state: &mut State) {
        tracing::info!("Option 3 Message");
    }

    fn handle_option4_message(_state: &mut State) {
        tracing::info!("Option 4 Message");
    }
}
