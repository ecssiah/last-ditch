pub mod action;
pub mod block;
pub mod state;

use action::{Action, WorldAction};
use block::Block;
use rand::Rng;
use state::{State, Leader, Entities, World};
use std::{
    sync::{Arc, RwLock},
    thread,
    time::{Duration, Instant},
};
use crate::{consts::{CHUNK_DIM, CHUNK_HALF, CHUNK_SIZE}, ActionReceiver};

const SIMULATION_SLEEP: u64 = 16;

pub struct Simulation {
    state: Arc<State>,
    action_rx: ActionReceiver,
}

impl Simulation {
    pub fn new(action_rx: ActionReceiver) -> Simulation {
        let state = Arc::new(State {
            leader: Arc::new(RwLock::new(Leader {
                name: "Michael".to_string(),
            })),
            entities: Arc::new(RwLock::new(Entities {})),
            world: Arc::new(RwLock::new(World {
                active: true,
                seed: 1234546789,
                time: 0.0,
                blocks: generate_blocks(),
            })),
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
                },
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

fn generate_blocks() -> [Block; CHUNK_SIZE] {
    core::array::from_fn(move |index| {
        let (x, y, z) = index_to_position(index);

        Block {
            position: [x as i32, y as i32, z as i32],
            color: [0.1, 0.3, 0.6, 1.0],
        }
    })
}

fn index_to_position(index: usize) -> (isize, isize, isize) {
    let x = (index % CHUNK_DIM) as isize - CHUNK_HALF;
    let y = ((index / CHUNK_DIM) % CHUNK_DIM) as isize - CHUNK_HALF;
    let z = (index / (CHUNK_DIM * CHUNK_DIM)) as isize - CHUNK_HALF;

    (x, y, z)
}

fn position_to_index(x: isize, y: isize, z: isize) -> Option<usize> {
    let x = (x + CHUNK_HALF) as usize;
    let y = (y + CHUNK_HALF) as usize;
    let z = (z + CHUNK_HALF) as usize;

    if x < CHUNK_DIM && y < CHUNK_DIM && z < CHUNK_DIM {
        Some(x + y * CHUNK_DIM + z * CHUNK_DIM * CHUNK_DIM)
    } else {
        None
    }
}