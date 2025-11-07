//! Current state of the simulation

pub mod admin;
pub mod physics;
pub mod population;
pub mod receiver;
pub mod time;
pub mod world;

pub use admin::Admin;
pub use physics::Physics;
pub use population::Population;
pub use receiver::Receiver;
pub use time::Time;
pub use world::World;

use crate::simulation::{
    self, constructor,
    consts::PROJECT_TITLE,
    state::{
        population::entity::Judge,
        receiver::action::{Action, AdminAction, JudgeAction, TestAction},
    },
};

pub struct State {
    pub simulation_kind: simulation::Kind,
    pub construct_rx: Option<tokio::sync::mpsc::Receiver<(World, Population)>>,
    pub admin: Admin,
    pub time: Time,
    pub physics: Physics,
    pub world: World,
    pub population: Population,
}

impl State {
    pub fn new(simulation_kind: simulation::Kind) -> Self {
        let construct_rx = None;

        let admin = Admin::new();
        let time = Time::new();
        let physics = Physics::new();
        let world = World::new(simulation_kind);
        let population = Population::new(simulation_kind);

        Self {
            simulation_kind,
            construct_rx,
            admin,
            time,
            physics,
            world,
            population,
        }
    }

    pub fn tick(action_vec: Vec<Action>, state: &mut State) {
        match state.admin.mode {
            admin::Mode::Menu => Self::tick_menu(action_vec, state),
            admin::Mode::Load => Self::tick_load(state),
            admin::Mode::Simulate => Self::tick_simulate(action_vec, state),
            admin::Mode::Shutdown => Self::tick_shutdown(state),
        }
    }

    fn tick_menu(action_vec: Vec<Action>, state: &mut State) {
        for action in action_vec {
            match action {
                Action::Admin(admin_action) => match admin_action {
                    AdminAction::Start => Self::init_load(state),
                    AdminAction::Quit => Self::init_shutdown(state),
                    _ => log::warn!("Received an invalid AdminAction in Menu mode: {:?}", action),
                },
                _ => log::warn!("Received an invalid Action in Menu mode: {:?}", action),
            }
        }
    }

    fn init_load(state: &mut State) {
        let simulation_kind = state.simulation_kind;

        let world = std::mem::replace(&mut state.world, World::placeholder());
        let population = std::mem::replace(&mut state.population, Population::placeholder());

        let (construct_tx, construct_rx) = tokio::sync::mpsc::channel(1);

        tokio::task::spawn_blocking(move || {
            let mut world = world;
            let mut population = population;

            constructor::world::run(simulation_kind, &mut world);
            constructor::population::run(simulation_kind, &world, &mut population);

            let _ = construct_tx.blocking_send((world, population));
        });

        state.construct_rx = Some(construct_rx);

        state.admin.mode = admin::Mode::Load;
        state.admin.message = "Construction in Progress...".to_string();
    }

    fn tick_load(state: &mut State) {
        if let Some(construct_rx) = &mut state.construct_rx {
            if let Ok((world, population)) = construct_rx.try_recv() {
                state.world = world;
                state.population = population;

                state.admin.mode = admin::Mode::Simulate;
                state.admin.message = format!("{} {}", PROJECT_TITLE, env!("CARGO_PKG_VERSION"));
            }
        }
    }

    fn tick_simulate(action_vec: Vec<Action>, state: &mut State) {
        Self::apply_simulate_action_vec(state, action_vec);

        Time::tick(&mut state.time);
        Population::tick(&state.world, &mut state.population);
        Physics::tick(&state.physics, &state.world, &mut state.population);
    }

    fn apply_simulate_action_vec(state: &mut State, action_vec: Vec<Action>) {
        for action in action_vec {
            match action {
                Action::Admin(admin_action) => match admin_action {
                    AdminAction::Debug => {
                        state.admin.debug_active = !state.admin.debug_active;
                    }
                    AdminAction::Quit => Self::init_shutdown(state),
                    _ => {
                        log::warn!(
                            "Received an invalid AdminAction in Simulate mode: {:?}",
                            action
                        );
                    }
                },
                Action::Judge(judge_action) => match judge_action {
                    JudgeAction::Movement(movement_data) => {
                        Judge::apply_movement_data(&movement_data, &mut state.population.judge);
                    }
                    JudgeAction::Jump(jump_action) => {
                        Judge::apply_jump_action(&jump_action, &mut state.population.judge);
                    }
                },
                Action::Test(test_action) => match test_action {
                    TestAction::Test1 => println!("Test Action 1"),
                    TestAction::Test2 => println!("Test Action 2"),
                    TestAction::Test3 => println!("Test Action 3"),
                    TestAction::Test4 => println!("Test Action 4"),
                },
            }
        }
    }

    fn init_shutdown(state: &mut State) {
        log::info!("Simulation Shutdown");

        state.admin.mode = admin::Mode::Shutdown;
    }

    fn tick_shutdown(state: &mut State) {
        state.admin.message = "Shutting down...".to_string();
    }
}
