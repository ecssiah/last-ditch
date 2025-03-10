pub mod action;
pub mod agent;
pub mod block;
pub mod chunk;
pub mod state;
pub mod world;

use action::{Action, AgentAction, WorldAction};
use agent::Agent;
use block::{Block, Kind};
use chunk::Chunk;
use glam::{IVec3, Quat, Vec3};
use noise::{NoiseFn, Perlin};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;
use state::State;
use std::{
    sync::{Arc, RwLock, WaitTimeoutResult},
    thread,
    time::{Duration, Instant},
};
use tokio::sync::mpsc::UnboundedReceiver;
use wgpu::Color;
use world::World;

pub const SEED: u64 = 101;
pub const SIMULATION_WAIT: u64 = 16;
pub const UPDATE_WINDOW: u32 = 2;

pub const DEFAULT_LINEAR_SPEED: f32 = 22.0;
pub const DEFAULT_STRAFE_SPEED: f32 = 22.0;
pub const DEFAULT_ANGULAR_SPEED: f32 = 1.0;

pub const BLOCK_RADIUS: f32 = 0.5;
pub const BLOCK_SIZE: f32 = 2.0 * BLOCK_RADIUS;

pub const CHUNK_RADIUS: u32 = 1;
pub const CHUNK_SIZE: u32 = 2 * CHUNK_RADIUS + 1;
pub const CHUNK_AREA: u32 = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_VOLUME: u32 = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

pub const WORLD_RADIUS: u32 = 1;
pub const WORLD_SIZE: u32 = 2 * WORLD_RADIUS + 1;
pub const WORLD_AREA: u32 = WORLD_SIZE * WORLD_SIZE;
pub const WORLD_VOLUME: u32 = WORLD_SIZE * WORLD_SIZE * WORLD_SIZE;

pub const WORLD_BOUNDARY: u32 = CHUNK_RADIUS + WORLD_RADIUS * (2 * CHUNK_RADIUS);

pub struct Simulation {
    action_rx: UnboundedReceiver<Action>,
    state: Arc<State>,
}

impl Simulation {
    pub fn new(action_rx: UnboundedReceiver<Action>) -> Simulation {
        let state = Arc::new(State {
            agent: Simulation::setup_agent(),
            world: Simulation::setup_world(),
            blocks: Simulation::setup_blocks(),
            chunks: Simulation::setup_chunks(),
        });

        let simulation = Simulation { action_rx, state };

        simulation
    }

    pub fn generate(&mut self) {
        self.set_kind(IVec3::new(0, 0, 0), Kind::Metal);

        self.state.world.write().unwrap().update_window = UPDATE_WINDOW;

        println!("{:?}", self.state.chunks[13].read().unwrap().blocks);
    }

    fn update(&mut self, dt: f32) {
        self.process_actions();
        self.evolve(dt);
        self.track_modifications();
    }

    pub fn get_state(&self) -> Arc<State> {
        self.state.clone()
    }

    fn track_modifications(&mut self) {
        let mut world = self.state.world.write().unwrap();

        if world.update_window > 0 {
            world.update_window -= 1;
        } else {
            for chunk in self.state.chunks.iter() {
                let mut chunk = chunk.write().unwrap();
    
                chunk.modified = false;
            }
        }

        println!("Update Window: {:?}", world.update_window);
    }

    fn process_actions(&mut self) {
        while let Ok(action) = self.action_rx.try_recv() {
            match action {
                Action::World(WorldAction::Quit) => {
                    let mut world = self.state.world.write().unwrap();

                    world.active = false;
                }
                Action::Agent(AgentAction::Move(move_actions)) => {
                    let mut agent = self.state.agent.write().unwrap();

                    agent.speed = move_actions.forward + move_actions.backward;
                    agent.strafe_speed = move_actions.left + move_actions.right;
                }
                Action::Agent(AgentAction::Rotate(rotate_actions)) => {
                    let mut agent = self.state.agent.write().unwrap();

                    agent.look_yaw += rotate_actions.yaw;
                    agent.look_pitch -= rotate_actions.pitch;

                    agent.look_pitch = agent
                        .look_pitch
                        .clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());

                    let yaw_quat = Quat::from_rotation_y(agent.look_yaw);
                    let pitch_quat = Quat::from_rotation_x(agent.look_pitch);
                    let target_rotation = yaw_quat * pitch_quat;

                    agent.look_rotation = agent.look_rotation.slerp(target_rotation, 0.3);

                    agent.move_yaw = agent.look_yaw;
                }
            }
        }
    }

    fn evolve(&mut self, dt: f32) {
        let mut state = self.state.world.write().unwrap();
        state.time += dt;

        let mut agent = self.state.agent.write().unwrap();

        let yaw_quat = Quat::from_rotation_y(agent.move_yaw);

        let forward = yaw_quat * Vec3::Z;
        let right = yaw_quat * Vec3::X;

        let movement = forward * agent.speed + right * agent.strafe_speed;

        agent.position += dt * movement;
    }

    pub fn run(&mut self) {
        let mut previous_instant = Instant::now();

        loop {
            let now = Instant::now();

            let dt = now.duration_since(previous_instant).as_secs_f32();
            previous_instant = now;

            self.update(dt);

            thread::sleep(Duration::from_millis(SIMULATION_WAIT));
        }
    }

    fn setup_agent() -> Arc<RwLock<Agent>> {
        Arc::from(RwLock::from(Agent {
            id: 0,
            name: "Melchizedek".to_string(),
            position: Vec3::new(0.0, 16.0, -16.0),
            speed: 0.0,
            strafe_speed: 0.0,
            angular_speed: 0.0,
            move_yaw: 0.0,
            look_pitch: 0.0,
            look_yaw: 0.0,
            look_rotation: Quat::IDENTITY,
        }))
    }

    fn setup_world() -> Arc<RwLock<World>> {
        Arc::from(RwLock::from(World {
            active: true,
            update_window: 0,
            seed: SEED,
            time: 0.0,
        }))
    }

    fn setup_blocks() -> Arc<[Block]> {
        Arc::from([
            Block {
                kind: block::Kind::Air,
                opacity: 1.0,
                color: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0,
                },
            },
            Block {
                kind: block::Kind::Metal,
                opacity: 1.0,
                color: Color {
                    r: 0.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                },
            },
            Block {
                kind: block::Kind::Concrete,
                opacity: 1.0,
                color: Color {
                    r: 1.0,
                    g: 0.0,
                    b: 1.0,
                    a: 1.0,
                },
            },
        ])
    }

    fn setup_chunks() -> Arc<[Arc<RwLock<Chunk>>]> {
        let chunks: [Arc<RwLock<Chunk>>; WORLD_VOLUME as usize] =
            core::array::from_fn(|chunk_id| {
                Arc::from(RwLock::from(Chunk {
                    modified: false,
                    local_position: Simulation::chunk_id_to_position(chunk_id as u32),
                    world_position: (CHUNK_SIZE as i32
                        * Simulation::chunk_id_to_position(chunk_id as u32))
                    .as_vec3(),
                    palette: Vec::from([Kind::Air]),
                    blocks: Box::new([0; CHUNK_VOLUME as usize]),
                }))
            });

        Arc::from(chunks)
    }

    fn set_kind(&mut self, grid_position: IVec3, kind: Kind) {
        if let Some(chunk_id) = Simulation::get_chunk_id(&grid_position) {
            let chunk = &self.state.chunks[chunk_id as usize];

            {
                let chunk = chunk.read().unwrap();

                if let Some(block_id) = Simulation::get_block_id(&grid_position) {
                    let current_index = chunk.blocks[block_id as usize];

                    if chunk.palette.get(current_index as usize) == Some(&kind) {
                        return;
                    }
                }
            }

            let mut chunk = chunk.write().unwrap();

            if let Some(block_id) = Simulation::get_block_id(&grid_position) {
                let kind_index = match chunk
                    .palette
                    .iter()
                    .position(|target_kind| *target_kind == kind)
                {
                    Some(i) => i as u32,
                    None => {
                        chunk.palette.push(kind);
                        (chunk.palette.len() - 1) as u32
                    }
                };

                chunk.blocks[block_id as usize] = kind_index;
            }
        }
    }

    pub fn chunk_id_to_position(id: u32) -> IVec3 {
        let x = (id % WORLD_SIZE) as i32 - WORLD_RADIUS as i32;
        let y = (id / WORLD_AREA) as i32 - WORLD_RADIUS as i32;
        let z = (id / WORLD_SIZE % WORLD_SIZE) as i32 - WORLD_RADIUS as i32;

        IVec3::new(x, y, z)
    }

    pub fn block_id_to_position(id: u32) -> IVec3 {
        let x = (id % CHUNK_SIZE) as i32 - CHUNK_RADIUS as i32;
        let y = (id / CHUNK_AREA) as i32 - CHUNK_RADIUS as i32;
        let z = (id / CHUNK_SIZE % CHUNK_SIZE) as i32 - CHUNK_RADIUS as i32;

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

    fn get_chunk_id(grid_position: &IVec3) -> Option<u32> {
        if Simulation::is_on_map(grid_position) {
            let grid_position_normalized = grid_position + IVec3::splat(CHUNK_RADIUS as i32);

            let chunk_position =
                (grid_position_normalized).div_euclid(IVec3::splat(CHUNK_SIZE as i32));

            Some(
                ((chunk_position.x + WORLD_RADIUS as i32) as u32
                    + (chunk_position.z + WORLD_RADIUS as i32) as u32 * WORLD_SIZE
                    + (chunk_position.y + WORLD_RADIUS as i32) as u32 * WORLD_AREA)
                    as u32,
            )
        } else {
            None
        }
    }

    fn get_block_id(grid_position: &IVec3) -> Option<u32> {
        if Simulation::is_on_map(grid_position) {
            let grid_position_normalized = grid_position + IVec3::splat(CHUNK_RADIUS as i32);

            let local_block_position =
                grid_position_normalized.rem_euclid(IVec3::splat(CHUNK_SIZE as i32));

            Some(
                (local_block_position.x as u32
                    + local_block_position.z as u32 * CHUNK_SIZE
                    + local_block_position.y as u32 * CHUNK_AREA) as u32,
            )
        } else {
            None
        }
    }

    fn get_grid_position(world_position: &Vec3) -> IVec3 {
        (world_position + Vec3::splat(BLOCK_RADIUS))
            .floor()
            .as_ivec3()
    }

    fn is_on_map(grid_position: &IVec3) -> bool {
        let in_x_range =
            grid_position.x >= -(WORLD_BOUNDARY as i32) && grid_position.x <= WORLD_BOUNDARY as i32;
        let in_y_range =
            grid_position.y >= -(WORLD_BOUNDARY as i32) && grid_position.y <= WORLD_BOUNDARY as i32;
        let in_z_range =
            grid_position.z >= -(WORLD_BOUNDARY as i32) && grid_position.z <= WORLD_BOUNDARY as i32;

        in_x_range && in_y_range && in_z_range
    }
}
