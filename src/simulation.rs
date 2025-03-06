pub mod action;
pub mod state;

use action::{Action, WorldAction};
use state::{State, User, World};
use std::{
    sync::{Arc, RwLock},
    thread,
    time::{Duration, Instant},
};

use crate::ActionReceiver;

const SIMULATION_SLEEP: u64 = 16;

pub struct Simulation {
    state: Arc<State>,
    action_rx: ActionReceiver,
}

impl Simulation {
    pub fn new(action_rx: ActionReceiver) -> Simulation {
        let state = Arc::new(State {
            world: Arc::new(RwLock::new(World {
                active: true,
                seed: 1234546789,
                time: 0.0,
            })),
            user: Arc::new(RwLock::new(User {})),
        });

        Simulation { state, action_rx }
    }

    pub fn get_state(&self) -> Arc<State> {
        return self.state.clone();
    }

    fn update(&mut self, dt: f64) {
        self.process_actions();

        let mut state = self.state.world.write().unwrap();

        state.time += dt;
    }

    fn process_actions(&mut self) {
        while let Ok(action) = self.action_rx.try_recv() {
            let mut world = self.state.world.write().unwrap();

            match action {
                Action::World(world_action) => match world_action {
                    WorldAction::Quit => {
                        world.active = false;
                    }
                }
            }
        }
    }

    pub fn run(&mut self) {
        let mut previous_instant = Instant::now();

        loop {
            let now = Instant::now();
            let dt = now.duration_since(previous_instant).as_secs_f64();
            previous_instant = now;

            self.update(dt);
            thread::sleep(Duration::from_millis(SIMULATION_SLEEP));
        }
    }
}
