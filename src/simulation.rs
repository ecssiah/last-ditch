pub mod action;
pub mod block;
pub mod chunk;
pub mod state;

use crate::ActionReceiver;
use action::{Action, EntityAction, WorldAction};
use block::{Block, BlockType};
use chunk::Chunk;
use glam::{IVec3, Quat, Vec3};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;
use state::{Entity, State, World};
use std::{
    sync::{Arc, RwLock},
    thread,
    time::{Duration, Instant},
};
use wgpu::Color;

pub const DEFAULT_SEED: u64 = 101;
pub const SIMULATION_SLEEP: u64 = 16;

pub const DEFAULT_LINEAR_SPEED: f32 = 10.0;
pub const DEFAULT_STRAFE_SPEED: f32 = 10.0;
pub const DEFAULT_ANGULAR_SPEED: f32 = 1.0;

pub const WORLD_RADIUS: u32 = 5;
pub const WORLD_SIZE: u32 = 2 * WORLD_RADIUS + 1;
pub const WORLD_AREA: u32 = WORLD_SIZE * WORLD_SIZE;
pub const WORLD_VOLUME: u32 = WORLD_SIZE * WORLD_SIZE * WORLD_SIZE;

pub const CHUNK_RADIUS: u32 = 5;
pub const CHUNK_SIZE: u32 = 2 * CHUNK_RADIUS + 1;
pub const CHUNK_AREA: u32 = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_VOLUME: u32 = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

pub struct Simulation {
    state: Arc<State>,
    action_rx: ActionReceiver,
}

impl Simulation {
    pub fn new(action_rx: ActionReceiver) -> Simulation {
        let entity = Entity {
            id: 0,
            name: "Melchizedek".to_string(),
            position: Vec3::new(0.0, 0.0, -64.0),
            speed: 0.0,
            strafe_speed: 0.0,
            angular_speed: 0.0,
            move_yaw: 0.0,
            look_pitch: 0.0,
            look_yaw: 0.0,
            look_rotation: Quat::IDENTITY,
        };

        let world = World {
            active: true,
            seed: DEFAULT_SEED,
            time: 0.0,
            chunks: Simulation::generate_chunks(),
        };

        let state = Arc::new(State {
            entity: Arc::new(RwLock::new(entity)),
            world: Arc::new(RwLock::new(world)),
        });

        Simulation { state, action_rx }
    }

    pub fn get_state(&self) -> Arc<State> {
        return self.state.clone();
    }

    fn update(&mut self, dt: f32) {
        self.process_actions();
        self.evolve(dt);
    }

    fn process_actions(&mut self) {
        while let Ok(action) = self.action_rx.try_recv() {
            match action {
                Action::World(WorldAction::Quit) => {
                    let mut world = self.state.world.write().unwrap();

                    world.active = false;
                }
                Action::Entity(EntityAction::Move(move_actions)) => {
                    let mut entity = self.state.entity.write().unwrap();

                    entity.speed = move_actions.forward + move_actions.backward;
                    entity.strafe_speed = move_actions.left + move_actions.right;
                }
                Action::Entity(EntityAction::Rotate(rotate_actions)) => {
                    let mut entity = self.state.entity.write().unwrap();

                    entity.look_yaw += rotate_actions.yaw;
                    entity.look_pitch -= rotate_actions.pitch;

                    entity.look_pitch = entity
                        .look_pitch
                        .clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());

                    let yaw_quat = Quat::from_rotation_y(entity.look_yaw);
                    let pitch_quat = Quat::from_rotation_x(entity.look_pitch);
                    let target_rotation = yaw_quat * pitch_quat;

                    entity.look_rotation = entity.look_rotation.slerp(target_rotation, 0.3);

                    entity.move_yaw = entity.look_yaw;
                }
            }
        }
    }

    fn evolve(&mut self, dt: f32) {
        let mut state = self.state.world.write().unwrap();
        state.time += dt;

        let mut entity = self.state.entity.write().unwrap();

        let yaw_quat = Quat::from_rotation_y(entity.move_yaw);

        let forward = yaw_quat * Vec3::Z;
        let right = yaw_quat * Vec3::X;

        let movement = forward * entity.speed + right * entity.strafe_speed;

        entity.position += dt * movement;
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
                let chunk_world_position = (CHUNK_SIZE as i32 * chunk_local_position).as_vec3();

                let blocks: [Block; CHUNK_VOLUME as usize] = core::array::from_fn(|block_id| {
                    let block_id = block_id as u32;
                    let block_type: BlockType;
                    let block_local_position = Simulation::block_id_to_position(block_id);
                    let block_world_position =
                        chunk_world_position + block_local_position.as_vec3();
                    let block_color: Color;

                    let roll = rng.gen::<f32>();

                    if roll < 0.001 {
                        block_type = BlockType::Solid;
                        block_color = Color {
                            r: rng.gen::<f64>(),
                            g: rng.gen::<f64>(),
                            b: rng.gen::<f64>(),
                            a: 1.0,
                        };
                    // } else if roll < 0.75 {
                    //     block_type = BlockType::Translucent;
                    //     block_color = Color {
                    //         r: 0.2,
                    //         g: 0.4,
                    //         b: 0.5,
                    //         a: 0.1,
                    //     };
                    } else {
                        block_type = BlockType::Air;
                        block_color = Color::WHITE;
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

    fn chunk_id_to_position(id: u32) -> IVec3 {
        let x = (id % WORLD_SIZE) as i32 - WORLD_RADIUS as i32;
        let y = (id / WORLD_SIZE % WORLD_SIZE) as i32 - WORLD_RADIUS as i32;
        let z = (id / WORLD_AREA) as i32 - WORLD_RADIUS as i32;

        IVec3::new(x, y, z)
    }

    fn block_id_to_position(id: u32) -> IVec3 {
        let x = (id % CHUNK_SIZE) as i32 - CHUNK_RADIUS as i32;
        let y = (id / CHUNK_SIZE % CHUNK_SIZE) as i32 - CHUNK_RADIUS as i32;
        let z = (id / CHUNK_AREA) as i32 - CHUNK_RADIUS as i32;

        IVec3::new(x, y, z)
    }

    fn chunk_position_to_id(position: &IVec3) -> u32 {
        let x = (position.x + WORLD_RADIUS as i32) as u32;
        let y = (position.y + WORLD_RADIUS as i32) as u32;
        let z = (position.z + WORLD_RADIUS as i32) as u32;

        x + y * WORLD_SIZE + z * WORLD_AREA
    }

    fn block_position_to_id(position: &IVec3) -> u32 {
        let x = (position.x + CHUNK_RADIUS as i32) as u32;
        let y = (position.y + CHUNK_RADIUS as i32) as u32;
        let z = (position.z + CHUNK_RADIUS as i32) as u32;

        x + y * CHUNK_SIZE + z * CHUNK_AREA
    }
}
