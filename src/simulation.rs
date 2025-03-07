pub mod action;
pub mod block;
pub mod chunk;
pub mod state;

use crate::{
    consts::{
        CHUNK_AREA, CHUNK_RADIUS, CHUNK_SIZE, CHUNK_VOLUME, DEFAULT_LINEAR_SPEED, DEFAULT_SEED, SIMULATION_SLEEP, WORLD_AREA, WORLD_RADIUS, WORLD_SIZE, WORLD_VOLUME
    },
    ActionReceiver,
};
use action::{Action, EntityAction, WorldAction};
use block::{Block, BlockType};
use cgmath::{EuclideanSpace, Point3, Vector3, Vector4, Zero};
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
            position: Point3 {
                x: 0.0,
                y: 6.0,
                z: 16.0,
            },
            linear_speed: Vector3::zero(),
            angular_speed: Vector3::zero(),
            direction: Vector3::zero(),
            facing: Vector3 {
                x: 0.0,
                y: -0.3,
                z: -1.0,
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

        let mut judge = self.state.judge.write().unwrap();
        judge.position = judge.position + DEFAULT_LINEAR_SPEED * judge.direction;
    }

    fn process_actions(&mut self) {
        while let Ok(action) = self.action_rx.try_recv() {
            match action {
                Action::World(WorldAction::Quit) => {
                    let mut world = self.state.world.write().unwrap();
                    world.active = false;
                }
                Action::Entity(EntityAction::Move(movement)) => {
                    let mut judge = self.state.judge.write().unwrap();
                    judge.direction = movement;
                }
                Action::Entity(EntityAction::Rotate(rotation)) => {
                    let mut judge = self.state.judge.write().unwrap();
                    judge.facing = rotation;
                }
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

        (0..WORLD_VOLUME)
            .map(|chunk_id| {
                let chunk_local_position = Simulation::chunk_id_to_position(chunk_id);
                let chunk_world_position = (CHUNK_SIZE as i32 * chunk_local_position)
                    .cast::<f32>()
                    .unwrap();

                let blocks: [Block; CHUNK_VOLUME as usize] = core::array::from_fn(|block_id| {
                    let block_id = block_id as u32;

                    let block_local_position = Simulation::block_id_to_position(block_id);
                    let block_world_position =
                        chunk_world_position + block_local_position.cast::<f32>().unwrap().to_vec();

                    let roll = rng.gen::<f32>();

                    let mut block_type = BlockType::Solid;
                    let mut block_color = Vector4::new(0.0, 0.0, 0.0, 1.0);

                    // if block_world_position.x < 0.0 {
                    //     block_color.x = rng.gen_range(0.0..0.5);
                    // } else if block_world_position.x > 0.0 {
                    //     block_color.x = rng.gen_range(0.5..=1.0);
                    // } else {
                    //     block_color.y = 1.0;
                    // }

                    if roll < 0.5 {
                        block_type = BlockType::Solid;
                        block_color =
                            Vector4::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>(), 1.0);
                    // } else if roll < 0.100 {
                    //     block_type = BlockType::Translucent;
                    //     block_color = Vector4::new(0.3, 0.5, 0.7, 0.2);
                    } else {
                        block_type = BlockType::None;
                        block_color = Vector4::new(1.0, 1.0, 1.0, 1.0);
                    }

                    Block {
                        id: block_id,
                        chunk_id,
                        block_type,
                        local_position: block_local_position,
                        world_position: block_world_position,
                        color: block_color,
                    }
                });

                Chunk {
                    id: chunk_id,
                    local_position: chunk_local_position,
                    world_position: chunk_world_position,
                    modified: true,
                    blocks: Box::new(blocks),
                }
            })
            .collect()
    }

    fn chunk_id_to_position(id: u32) -> Point3<i32> {
        let x = (id % WORLD_SIZE) as i32 - WORLD_RADIUS as i32;
        let y = (id / WORLD_SIZE % WORLD_SIZE) as i32 - WORLD_RADIUS as i32;
        let z = (id / WORLD_AREA) as i32 - WORLD_RADIUS as i32;

        Point3 { x, y, z }
    }

    fn block_id_to_position(id: u32) -> Point3<i32> {
        let x = (id % CHUNK_SIZE) as i32 - CHUNK_RADIUS as i32;
        let y = (id / CHUNK_SIZE % CHUNK_SIZE) as i32 - CHUNK_RADIUS as i32;
        let z = (id / CHUNK_AREA) as i32 - CHUNK_RADIUS as i32;

        Point3 { x, y, z }
    }

    fn chunk_position_to_id(position: &Point3<i32>) -> u32 {
        let x = (position.x + WORLD_RADIUS as i32) as u32;
        let y = (position.y + WORLD_RADIUS as i32) as u32;
        let z = (position.z + WORLD_RADIUS as i32) as u32;

        x + y * WORLD_SIZE + z * WORLD_AREA
    }

    fn block_position_to_id(position: &Vector3<i32>) -> u32 {
        let x = (position.x + CHUNK_RADIUS as i32) as u32;
        let y = (position.y + CHUNK_RADIUS as i32) as u32;
        let z = (position.z + CHUNK_RADIUS as i32) as u32;

        x + y * CHUNK_SIZE + z * CHUNK_AREA
    }
}
