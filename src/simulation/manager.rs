pub mod message;
pub mod status;
pub mod timestep;
pub mod viewer;

pub use message::Message;
pub use timestep::Timestep;
pub use viewer::Viewer;

use crate::simulation::{
    constants::{SIMULATION_MAX_TICKS_PER_FRAME, SIMULATION_TICK_DURATION},
    manager::{status::Status, viewer::View},
    state::{
        action::{
            act::{self},
            Act,
        },
        work::{
            population_task::{construct_population_data::ConstructPopulationData, PopulationTask},
            population_worker::PopulationWorker,
            world_task::{construct_world_data::ConstructWorldData, WorldTask},
            world_worker::WorldWorker,
        },
        world::block::Kind,
        State,
    },
};
use std::time::{Duration, Instant};

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

    pub fn start(manager: &mut Manager) {
        manager.timestep.ticks_frame = 0;
    }

    pub fn has_work(manager: &Manager) -> bool {
        Instant::now() >= manager.timestep.next_instant
            && manager.timestep.ticks_frame < SIMULATION_MAX_TICKS_PER_FRAME
    }

    pub fn tick(state: &mut State, manager: &mut Manager) -> bool {
        match manager.status {
            Status::Run => State::tick(state),
            Status::Done => return false,
        }

        Manager::handle_messages(state, manager);
        Viewer::tick(manager, state);
        Manager::update_timestep(manager);

        true
    }

    fn handle_messages(state: &mut State, manager: &mut Manager) {
        while let Ok(message) = manager.message_rx.try_recv() {
            Manager::handle_message(&message, state, manager);
        }
    }

    fn handle_message(message: &Message, state: &mut State, manager: &mut Manager) {
        match message {
            Message::Interact1 => Self::handle_interact1(state),
            Message::Interact2 => Self::handle_interact2(state),
            Message::Rotate(rotate_data) => Self::handle_rotate_message(rotate_data, state),
            Message::Move(move_data) => Self::handle_move_message(move_data, state),
            Message::Jump => Self::handle_jump_message(state),
            Message::Debug => todo!(),
            Message::Generate(generate_data) => Self::handle_generate_message(generate_data, state),
            Message::Quit => Self::handle_quit_message(state, manager),
            Message::Option1 => Self::handle_option1_message(state),
            Message::Option2 => Self::handle_option2_message(state),
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

    fn handle_generate_message(generate_data: &message::GenerateData, state: &mut State) {
        State::seed(generate_data.seed, state);

        let construct_world_data = ConstructWorldData { stage: 0 };
        let world_task = WorldTask::ConstructWorld(construct_world_data);

        WorldWorker::enqueue(world_task, &mut state.work.world_worker.task_deque);

        let construct_population_data = ConstructPopulationData { stage: 0 };
        let population_task = PopulationTask::ConstructPopulation(construct_population_data);

        PopulationWorker::enqueue(
            population_task,
            &mut state.work.population_worker.task_deque,
        );
    }

    fn handle_quit_message(_state: &mut State, manager: &mut Manager) {
        // TODO: Save Simulation State!

        manager.status = Status::Done;
    }

    fn handle_option1_message(state: &mut State) {
        state.population.judge.selected_block_kind =
            Kind::prev(state.population.judge.selected_block_kind);
    }

    fn handle_option2_message(state: &mut State) {
        state.population.judge.selected_block_kind =
            Kind::next(state.population.judge.selected_block_kind);
    }

    pub fn update_timestep(manager: &mut Manager) {
        manager.timestep.ticks_total += 1;
        manager.timestep.ticks_frame += 1;

        manager.timestep.next_instant = manager.timestep.start_instant
            + manager.timestep.ticks_total * SIMULATION_TICK_DURATION;
    }

    pub fn fix_timestep(manager: &mut Manager) {
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
