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
    state::receiver::action::{Action, AdminAction, JudgeAction, TestAction},
};

pub struct State {
    pub action_vec: Vec<Action>,
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
        let action_vec = Vec::default();
        let result_rx = None;

        let admin = Admin::new();
        let compute = Compute::new();
        let time = Time::new();
        let physics = Physics::new();
        let world = World::new(kind);
        let population = Population::new(kind);

        Self {
            action_vec,
            result_rx,
            admin,
            compute,
            time,
            physics,
            world,
            population,
        }
    }

    pub fn setup(&mut self) {
        self.admin.setup();
    }

    pub fn receive_action(&mut self, action: Action) {
        self.action_vec.push(action);
    }

    pub fn tick(&mut self) {
        match self.admin.mode {
            admin::Mode::Menu => {
                let action_vec = std::mem::take(&mut self.action_vec);

                for action in action_vec {
                    match action {
                        Action::Admin(admin_action) => match admin_action {
                            AdminAction::Start => {
                                let world =
                                    std::mem::replace(&mut self.world, World::placeholder());

                                let population = std::mem::replace(
                                    &mut self.population,
                                    Population::placeholder(),
                                );

                                let (result_tx, result_rx) = tokio::sync::mpsc::channel(1);

                                tokio::task::spawn_blocking(move || {
                                    let mut world = world;
                                    let mut population = population;

                                    world.setup();
                                    population.setup(&world);

                                    let _ = result_tx.blocking_send((world, population));
                                });

                                self.result_rx = Some(result_rx);

                                self.admin.mode = admin::Mode::Load;
                                self.admin.message = "Construction in Progress...".to_string();
                            }
                            AdminAction::Exit => (),
                        },
                        _ => log::warn!("Received an invalid action in Menu mode: {:?}", action),
                    }
                }
            }
            admin::Mode::Load => {
                let _action_vec = std::mem::take(&mut self.action_vec);

                if let Some(result_rx) = &mut self.result_rx {
                    if let Ok((world, population)) = result_rx.try_recv() {
                        self.world = world;
                        self.population = population;

                        self.admin.mode = admin::Mode::Simulate;
                        self.admin.message =
                            format!("{} {}", PROJECT_TITLE, env!("CARGO_PKG_VERSION"));
                    }
                }
            }
            admin::Mode::Simulate => {
                let action_vec = std::mem::take(&mut self.action_vec);

                for action in action_vec {
                    match action {
                        Action::Admin(admin_action) => match admin_action {
                            AdminAction::Exit => {
                                self.admin.mode = admin::Mode::Exit;
                                self.admin.message = "NO MESSAGE SET".to_string();
                            }
                            _ => {
                                log::warn!("Received an invalid action in Exit Mode: {:?}", action)
                            }
                        },
                        Action::Judge(judge_action) => {
                            let judge = self.population.get_judge_mut();

                            match judge_action {
                                JudgeAction::Movement(movement_data) => {
                                    judge.apply_movement_data(&movement_data);
                                }
                                JudgeAction::Jump(jump_action) => {
                                    judge.apply_jump_action(&jump_action);
                                }
                            }
                        }
                        Action::Test(test_action) => match test_action {
                            TestAction::Test1 => println!("Test Action 1"),
                            TestAction::Test2 => println!("Test Action 2"),
                            TestAction::Test3 => println!("Test Action 3"),
                            TestAction::Test4 => println!("Test Action 4"),
                        },
                    }
                }

                self.admin.tick();
                self.time.tick();
                self.world.tick();
                self.population.tick(&self.world);
                self.physics.tick(&self.world, &mut self.population);
                self.compute.tick(&self.world, &self.population);
            }
            admin::Mode::Shutdown => {
                let _action_vec = std::mem::take(&mut self.action_vec);
            }
            admin::Mode::Exit => {
                let _action_vec = std::mem::take(&mut self.action_vec);
            }
        }
    }
}
