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
use block::{Block, BlockType};
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
        let judge = Judge {
            name: "Melchizedek".to_string(),
            position: Vector3 {
                x: 32.0,
                y: 32.0,
                z: 32.0,
            },
            direction: Vector3 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
        };

        let entities = Entities {};

        let world = World {
            active: true,
            seed: DEFAULT_SEED,
            time: 0.0,
            chunks: Simulation::generate_chunks(),
        };

        let state = Arc::new(State {
            judge: Arc::new(RwLock::new(judge)),
            entities: Arc::new(RwLock::new(entities)),
            world: Arc::new(RwLock::new(world)),
        });

        Simulation { state, action_rx }
    }

    pub fn get_state(&self) -> Arc<State> {
        return self.state.clone();
    }

    fn update(&mut self, dt: f32) {
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
            let dt = now.duration_since(previous_instant).as_secs_f32();
            previous_instant = now;

            self.update(dt);
            thread::sleep(Duration::from_millis(SIMULATION_SLEEP));
        }
    }

    fn generate_chunks() -> Vec<Chunk> {
        let mut rng = Pcg64::seed_from_u64(DEFAULT_SEED);

        let mut chunks = Vec::new();

        for chunk_id in 0..WORLD_VOLUME {
            let chunk_position = Simulation::chunk_id_to_position(chunk_id);

            let blocks: [Block; CHUNK_VOLUME as usize] = core::array::from_fn(|block_index| {
                let id = block_index as u32;

                let roll = rng.gen::<f32>();

                let block_type: BlockType;
                let color: Vector4<f32>;

                if roll < 0.050 {
                    block_type = BlockType::Solid;
                    color = Vector4::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>(), 1.0);
                } else if roll < 0.100 {
                    block_type = BlockType::Translucent;
                    color = Vector4::new(0.3, 0.5, 0.7, 0.2);
                } else {
                    block_type = BlockType::None;
                    color = Vector4::new(1.0, 1.0, 1.0, 1.0);
                }

                let position = Simulation::block_id_to_position(id);

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

    fn chunk_id_to_position(index: u32) -> Vector3<i32> {
        let x = (index % WORLD_SIZE) as i32 - WORLD_RADIUS as i32;
        let y = ((index / WORLD_SIZE) % WORLD_SIZE) as i32 - WORLD_RADIUS as i32;
        let z = (index / WORLD_AREA) as i32 - WORLD_RADIUS as i32;

        Vector3 { x, y, z }
    }

    fn block_id_to_position(index: u32) -> Vector3<i32> {
        let x = (index % CHUNK_SIZE) as i32 - CHUNK_RADIUS as i32;
        let y = ((index / CHUNK_SIZE) % CHUNK_SIZE) as i32 - CHUNK_RADIUS as i32;
        let z = (index / CHUNK_AREA) as i32 - CHUNK_RADIUS as i32;

        Vector3 { x, y, z }
    }

    fn chunk_position_to_id(x: i32, y: i32, z: i32) -> u32 {
        let x = (x + WORLD_RADIUS as i32) as u32;
        let y = (y + WORLD_RADIUS as i32) as u32;
        let z = (z + WORLD_RADIUS as i32) as u32;

        x + y * WORLD_SIZE as u32 + z * WORLD_AREA as u32
    }

    fn block_position_to_id(x: i32, y: i32, z: i32) -> u32 {
        let x = (x + CHUNK_RADIUS as i32) as u32;
        let y = (y + CHUNK_RADIUS as i32) as u32;
        let z = (z + CHUNK_RADIUS as i32) as u32;

        x + y * CHUNK_SIZE as u32 + z * CHUNK_AREA as u32
    }
}
