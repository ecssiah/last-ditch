//! Current state of the simulation

pub mod admin;
pub mod compute;
pub mod physics;
pub mod population;
pub mod receiver;
pub mod time;
pub mod world;

pub use admin::Admin;
pub use compute::Compute;
pub use physics::Physics;
pub use population::Population;
pub use receiver::Receiver;
pub use time::Time;
pub use world::World;

use crate::simulation::{
    self,
    consts::PROJECT_TITLE,
    state::{
        population::entity::Judge,
        receiver::action::{Action, AdminAction, JudgeAction, TestAction},
    },
};

pub struct State {
    pub kind: simulation::Kind,
    pub result_rx: Option<tokio::sync::mpsc::Receiver<(World, Population)>>,
    pub admin: Admin,
    pub compute: Compute,
    pub time: Time,
    pub physics: Physics,
    pub world: World,
    pub population: Population,
}

impl State {
    pub fn new(kind: simulation::Kind) -> Self {
        let result_rx = None;

        let admin = Admin::new();
        let compute = Compute::new();
        let time = Time::new();
        let physics = Physics::new();
        let world = World::new(kind);
        let population = Population::new(kind);

        Self {
            kind,
            result_rx,
            admin,
            compute,
            time,
            physics,
            world,
            population,
        }
    }

    pub fn tick(state: &mut State, action_vec: Vec<Action>) {
        match state.admin.mode {
            admin::Mode::Menu => Self::tick_menu(state, action_vec),
            admin::Mode::Load => Self::tick_load(state, action_vec),
            admin::Mode::Simulate => Self::tick_simulate(state, action_vec),
            admin::Mode::Shutdown => Self::tick_shutdown(state, action_vec),
        }
    }

    fn tick_menu(state: &mut State, action_vec: Vec<Action>) {
        for action in action_vec {
            match action {
                Action::Admin(admin_action) => match admin_action {
                    AdminAction::Start => Self::initialize_load(state),
                    _ => log::warn!("Received an invalid AdminAction in Menu mode: {:?}", action),
                },
                _ => log::warn!("Received an invalid Action in Menu mode: {:?}", action),
            }
        }
    }

    fn initialize_load(state: &mut State) {
        let kind = state.kind;
        let world = std::mem::replace(&mut state.world, World::placeholder());
        let population = std::mem::replace(&mut state.population, Population::placeholder());

        let (result_tx, result_rx) = tokio::sync::mpsc::channel(1);

        tokio::task::spawn_blocking(move || {
            let mut world = world;
            let mut population = population;

            World::setup(kind, &mut world);
            Population::setup(kind, &mut population, &world);

            let _ = result_tx.blocking_send((world, population));
        });

        state.result_rx = Some(result_rx);

        state.admin.mode = admin::Mode::Load;
        state.admin.message = "Construction in Progress...".to_string();
    }

    fn tick_load(state: &mut State, _action_vec: Vec<Action>) {
        if let Some(result_rx) = &mut state.result_rx {
            if let Ok((world, population)) = result_rx.try_recv() {
                state.world = world;
                state.population = population;

                state.admin.mode = admin::Mode::Simulate;
                state.admin.message = format!("{} {}", PROJECT_TITLE, env!("CARGO_PKG_VERSION"));
            }
        }
    }

    fn tick_simulate(state: &mut State, action_vec: Vec<Action>) {
        for action in action_vec {
            match action {
                Action::Judge(judge_action) => match judge_action {
                    JudgeAction::Movement(movement_data) => {
                        Judge::apply_movement_data(&mut state.population.judge, &movement_data);
                    }
                    JudgeAction::Jump(jump_action) => {
                        Judge::apply_jump_action(&mut state.population.judge, &jump_action);
                    }
                },
                Action::Test(test_action) => match test_action {
                    TestAction::Test1 => println!("Test Action 1"),
                    TestAction::Test2 => println!("Test Action 2"),
                    TestAction::Test3 => println!("Test Action 3"),
                    TestAction::Test4 => println!("Test Action 4"),
                },
                _ => {
                    log::warn!("Received an invalid Action in Simulate mode: {:?}", action)
                }
            }
        }

        Time::tick(&mut state.time);
        Population::tick(&mut state.population, &mut state.world);
        Physics::tick(&state.physics, &state.world, &mut state.population);
        Compute::tick(&mut state.compute, &mut state.population, &state.world);
    }

    fn tick_shutdown(_state: &mut State, _action_vec: Vec<Action>) {}
}
