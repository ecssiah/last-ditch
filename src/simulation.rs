pub mod action;
pub mod state;

use std::{sync::{Arc, RwLock}, thread, time::{Duration, Instant}};

use state::{State, SharedState};

pub struct Simulation {
    state: SharedState,
}

impl Simulation {
    pub fn new() -> Simulation {
        let state = Arc::new(RwLock::new(State {
            time: 0.0,
        }));

        Simulation {
            state,
        }
    }

    pub fn get_shared_state(&self) -> SharedState {
        Arc::clone(&self.state)
    }

    fn update(&mut self, dt: f64) {
        let mut world = self.state.write().unwrap();
        world.time += dt;

    }

    pub fn run(&mut self) {
        let mut last_update = Instant::now();

        loop {
            let now = Instant::now();
            let dt = now.duration_since(last_update).as_secs_f64();
            last_update = now;

            self.update(dt);
            thread::sleep(Duration::from_millis(16));
        }
    }
}
