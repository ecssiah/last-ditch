pub mod action;
pub mod block;
pub mod chunk;
pub mod state;

use crate::{
    consts::{
        CHUNK_AREA, CHUNK_RADIUS, CHUNK_SIZE, CHUNK_VOLUME, DEFAULT_SEED, SIMULATION_SLEEP,
        WORLD_AREA, WORLD_RADIUS, WORLD_SIZE, WORLD_VOLUME,
    },
    ActionReceiver,
};
use action::{Action, WorldAction};
use block::Block;
use cgmath::{Vector3, Vector4};
use chunk::Chunk;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;
use state::{Entities, Judge, State, World};
use std::{
    sync::{Arc, RwLock},
    thread,
    time::{Duration, Instant},
};

pub struct Simulation {
    state: Arc<State>,
    action_rx: ActionReceiver,
}

impl Simulation {
    pub fn new(action_rx: ActionReceiver) -> Simulation {
        let state = Arc::new(State {
            judge: Arc::new(RwLock::new(Judge {
                name: "Melchizedek".to_string(),
                position: Vector3 { x: 32.0, y: 32.0, z: 32.0 },
                direction: Vector3 { x: 1.0, y: 0.0, z: 1.0 },
            })),
            entities: Arc::new(RwLock::new(Entities {})),
            world: Arc::new(RwLock::new(World {
                active: true,
                seed: DEFAULT_SEED,
                time: 0.0,
                chunks: generate_chunks(),
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

fn generate_chunks() -> Vec<Chunk> {
    let mut rng = Pcg64::seed_from_u64(DEFAULT_SEED);

    let mut chunks = Vec::new();

    for chunk_id in 0..WORLD_VOLUME {
        let chunk_position = id_to_chunk_position(chunk_id);

        let blocks: [Block; CHUNK_VOLUME as usize] = core::array::from_fn(|block_index| {
            let id = block_index as u64;

            let roll = rng.gen::<f32>();
            let mut block_type = block::BlockType::Empty;

            if roll < 0.050 {
                block_type = block::BlockType::Solid;
            }

            let position = id_to_block_position(id);

            println!("{:?}", chunk_position + position);

            let color = Vector4::new(
                rng.gen::<f32>(),
                rng.gen::<f32>(),
                rng.gen::<f32>(),
                rng.gen_range(0.0..=1.0),
            );

            Block {
                id,
                chunk_id,
                block_type,
                position,
                color,
            }
        });

        let chunk = Chunk {
            id: chunk_id,
            position: chunk_position,
            modified: true,
            blocks: Box::new(blocks),
        };

        chunks.push(chunk);
    }

    chunks
}

fn id_to_chunk_position(index: u64) -> Vector3<i64> {
    let x = (index % WORLD_SIZE) as i64 - WORLD_RADIUS as i64;
    let y = ((index / WORLD_SIZE) % WORLD_SIZE) as i64 - WORLD_RADIUS as i64;
    let z = (index / WORLD_AREA) as i64 - WORLD_RADIUS as i64;

    Vector3 { x, y, z }
}

fn id_to_block_position(index: u64) -> Vector3<i64> {
    let x = (index % CHUNK_SIZE) as i64 - CHUNK_RADIUS as i64;
    let y = ((index / CHUNK_SIZE) % CHUNK_SIZE) as i64 - CHUNK_RADIUS as i64;
    let z = (index / CHUNK_AREA) as i64 - CHUNK_RADIUS as i64;

    Vector3 { x, y, z }
}

fn get_chunk_id(x: i64, y: i64, z: i64) -> u64 {
    let x = (x + WORLD_RADIUS as i64) as u64;
    let y = (y + WORLD_RADIUS as i64) as u64;
    let z = (z + WORLD_RADIUS as i64) as u64;

    x + y * WORLD_SIZE as u64 + z * WORLD_AREA as u64
}

fn get_block_id(x: i64, y: i64, z: i64) -> u64 {
    let x = (x + CHUNK_RADIUS as i64) as u64;
    let y = (y + CHUNK_RADIUS as i64) as u64;
    let z = (z + CHUNK_RADIUS as i64) as u64;

    x + y * CHUNK_SIZE as u64 + z * CHUNK_AREA as u64
}
