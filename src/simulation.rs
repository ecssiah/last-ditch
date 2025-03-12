pub mod action;
pub mod agent;
pub mod block;
pub mod chunk;
pub mod state;
pub mod structure;
pub mod world;

use crate::include_config;
use action::{Action, AgentAction, MoveActions, RotateActions, WorldAction};
use agent::Agent;
use block::Block;
use chunk::Chunk;
use glam::{IVec3, Quat, Vec3};
use once_cell::sync::Lazy;
use ron::from_str;
use state::State;
use std::collections::HashMap;
use std::{
    sync::{Arc, RwLock},
    thread,
    time::{Duration, Instant},
};
use structure::{Structure, StructureKind};
use tokio::sync::mpsc::UnboundedReceiver;
use world::World;

pub const SEED: u64 = 101;
pub const SIMULATION_WAIT: u64 = 16;
pub const UPDATE_WINDOW: u32 = 2;

pub const DEFAULT_Z_SPEED: f32 = 22.0;
pub const DEFAULT_X_SPEED: f32 = 22.0;
pub const DEFAULT_ANGULAR_SPEED: f32 = 1.0;

pub const BLOCK_RADIUS: f32 = 0.5;
pub const BLOCK_SIZE: f32 = 2.0 * BLOCK_RADIUS;
pub const BLOCK_AREA: f32 = BLOCK_SIZE * BLOCK_SIZE;
pub const BLOCK_VOLUME: f32 = BLOCK_SIZE * BLOCK_SIZE * BLOCK_SIZE;

pub const CHUNK_RADIUS: u32 = 3;
pub const CHUNK_SIZE: u32 = 2 * CHUNK_RADIUS + 1;
pub const CHUNK_AREA: u32 = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_VOLUME: u32 = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

pub const WORLD_RADIUS: u32 = 3;
pub const WORLD_SIZE: u32 = 2 * WORLD_RADIUS + 1;
pub const WORLD_AREA: u32 = WORLD_SIZE * WORLD_SIZE;
pub const WORLD_VOLUME: u32 = WORLD_SIZE * WORLD_SIZE * WORLD_SIZE;

pub const WORLD_BOUNDARY: u32 = CHUNK_RADIUS + WORLD_RADIUS * CHUNK_SIZE;

pub const BLOCK_CONFIG: &str = include_config!("blocks.ron");
pub const STRUCTURE_CONFIG: &str = include_config!("structures.ron");

pub static BLOCKS: Lazy<Vec<Block>> =
    Lazy::new(|| from_str(BLOCK_CONFIG).expect("Failed to parse Block data"));

pub static STRUCTURES: Lazy<HashMap<StructureKind, Structure>> =
    Lazy::new(|| from_str(STRUCTURE_CONFIG).expect("Failed to parse Structure data"));

pub struct Simulation {
    action_rx: UnboundedReceiver<Action>,
    state: Arc<State>,
}

impl Simulation {
    pub fn new(action_rx: UnboundedReceiver<Action>) -> Simulation {
        let state = Arc::new(State {
            agent: Simulation::setup_agent(),
            world: Simulation::setup_world(),
            chunks: Simulation::setup_chunks(),
        });

        let simulation = Simulation { action_rx, state };

        simulation
    }

    pub fn generate(&mut self) {
        self.generate_structure(IVec3::new(0, 0, 0), StructureKind::Swastika);
        self.generate_structure(IVec3::new(10, 0, 10), StructureKind::Swastika);
        self.generate_structure(IVec3::new(-10, 0, 10), StructureKind::Swastika);
        self.generate_structure(IVec3::new(10, 0, -10), StructureKind::Swastika);
        self.generate_structure(IVec3::new(-10, 0, -10), StructureKind::Swastika);
        
        self.generate_structure(IVec3::new(0, 24, 0), StructureKind::Swastika);
        self.generate_structure(IVec3::new(10, 24, 10), StructureKind::Swastika);
        self.generate_structure(IVec3::new(-10, 24, 10), StructureKind::Swastika);
        self.generate_structure(IVec3::new(10, 24, -10), StructureKind::Swastika);
        self.generate_structure(IVec3::new(-10, 24, -10), StructureKind::Swastika);
     
        self.state.world.write().unwrap().update_window = UPDATE_WINDOW;
    }

    fn generate_structure(&mut self, world_position: IVec3, structure_kind: StructureKind) {
        if STRUCTURES.contains_key(&structure_kind) {
            let structure = &STRUCTURES[&structure_kind];

            for block_data in &structure.blocks[..] {
                let position_array: [i32; 3] = block_data.position.as_slice().try_into().unwrap();
                let grid_position: IVec3 = world_position + IVec3::from(position_array);
    
                self.set_kind(grid_position, block_data.kind);
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

            thread::sleep(Duration::from_millis(SIMULATION_WAIT));
        }
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
    }

    fn process_actions(&mut self) {
        while let Ok(action) = self.action_rx.try_recv() {
            match action {
                Action::World(WorldAction::Quit) => {
                    self.process_quit_action();
                }
                Action::Agent(AgentAction::Move(move_actions)) => {
                    self.process_move_actions(&move_actions);
                }
                Action::Agent(AgentAction::Rotate(rotate_actions)) => {
                    self.process_rotate_actions(&rotate_actions);
                }
            }
        }
    }

    fn process_quit_action(&mut self) {
        let mut world = self.state.world.write().unwrap();

        world.active = false;
    }

    fn process_move_actions(&mut self, move_actions: &MoveActions) {
        let mut agent = self.state.agent.write().unwrap();

        agent.z_speed = move_actions.z_axis;
        agent.x_speed = move_actions.x_axis;
    }

    fn process_rotate_actions(&mut self, rotate_actions: &RotateActions) {
        let mut agent = self.state.agent.write().unwrap();

        agent.look_x_axis -= rotate_actions.x_axis;
        agent.look_y_axis += rotate_actions.y_axis;

        agent.look_x_axis = agent
            .look_x_axis
            .clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());

        let y_axis_quat = Quat::from_rotation_y(agent.look_y_axis);
        let x_axis_quat = Quat::from_rotation_x(agent.look_x_axis);

        let target_rotation = y_axis_quat * x_axis_quat;

        agent.look_rotation = agent.look_rotation.slerp(target_rotation, 0.3);
    }

    fn evolve(&mut self, dt: f32) {
        self.evolve_world(dt);
        self.evolve_agents(dt);
    }

    fn evolve_world(&mut self, dt: f32) {
        let mut state = self.state.world.write().unwrap();
        state.time += dt;
    }

    fn evolve_agents(&mut self, dt: f32) {
        let mut agent = self.state.agent.write().unwrap();

        let y_axis_quat = Quat::from_rotation_y(agent.look_y_axis);

        let agent_x_axis = y_axis_quat * Vec3::X;
        let agent_z_axis = y_axis_quat * Vec3::Z;

        let movement = agent.x_speed * agent_x_axis + agent.z_speed * agent_z_axis;

        agent.position += dt * movement;
    }

    fn setup_agent() -> Arc<RwLock<Agent>> {
        Arc::from(RwLock::from(Agent {
            id: 0,
            name: "Melchizedek",
            position: Vec3::new(0.0, 12.0, 0.0),
            x_speed: 0.0,
            z_speed: 0.0,
            look_x_axis: 0.0,
            look_y_axis: 0.0,
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

    fn setup_chunks() -> Arc<[Arc<RwLock<Chunk>>]> {
        let chunks: [Arc<RwLock<Chunk>>; WORLD_VOLUME as usize] =
            core::array::from_fn(|chunk_id| {
                Arc::from(RwLock::from(Chunk {
                    modified: false,
                    position: Simulation::chunk_id_to_position(chunk_id as u32),
                    palette: Vec::from([block::BlockKind::Air]),
                    blocks: Box::new([0; CHUNK_VOLUME as usize]),
                }))
            });

        Arc::from(chunks)
    }

    fn set_kind(&mut self, grid_position: IVec3, kind: block::BlockKind) {
        if let Some(chunk_id) = Simulation::grid_position_to_chunk_id(&grid_position) {
            if let Some(block_id) = Simulation::grid_position_to_block_id(&grid_position) {
                let mut chunk = self.state.chunks[chunk_id as usize].write().unwrap();

                let kind_id = self.get_palette_id(&mut chunk, kind);

                chunk.blocks[block_id as usize] = kind_id;
            }
        }
    }

    fn get_palette_id(&self, chunk: &mut Chunk, kind: block::BlockKind) -> u32 {
        match chunk
            .palette
            .iter()
            .position(|palette_kind| kind == *palette_kind)
        {
            Some(id) => id as u32,
            None => {
                chunk.palette.push(kind.clone());

                let id = chunk.palette.len() - 1;
                id as u32
            }
        }
    }

    pub fn chunk_id_to_position(chunk_id: u32) -> IVec3 {
        let chunk_position_shifted = IVec3::new(
            (chunk_id % WORLD_SIZE) as i32,
            (chunk_id / WORLD_SIZE % WORLD_SIZE) as i32,
            (chunk_id / WORLD_AREA) as i32,
        );

        let chunk_position = chunk_position_shifted - IVec3::splat(WORLD_RADIUS as i32);

        chunk_position
    }

    pub fn block_id_to_position(block_id: u32) -> IVec3 {
        let block_position_shifted = IVec3::new(
            (block_id % CHUNK_SIZE) as i32,
            (block_id / CHUNK_SIZE % CHUNK_SIZE) as i32,
            (block_id / CHUNK_AREA) as i32,
        );

        let block_position = block_position_shifted - IVec3::splat(CHUNK_RADIUS as i32);

        block_position
    }

    // fn chunk_position_to_id(chunk_position: &IVec3) -> u32 {
    //     let x = (chunk_position.x + WORLD_RADIUS as i32) as u32;
    //     let y = (chunk_position.y + WORLD_RADIUS as i32) as u32;
    //     let z = (chunk_position.z + WORLD_RADIUS as i32) as u32;

    //     x + y * WORLD_SIZE + z * WORLD_AREA
    // }

    // fn block_position_to_id(block_position: &IVec3) -> u32 {
    //     let x = (block_position.x + CHUNK_RADIUS as i32) as u32;
    //     let y = (block_position.y + CHUNK_RADIUS as i32) as u32;
    //     let z = (block_position.z + CHUNK_RADIUS as i32) as u32;

    //     x + y * CHUNK_SIZE + z * CHUNK_AREA
    // }

    pub fn grid_position_to_chunk_id(grid_position: &IVec3) -> Option<u32> {
        if Simulation::is_on_map(grid_position) {
            let chunk_position_shifted = grid_position.map(|coordinate| {
                let coordinate_shifted = coordinate + WORLD_BOUNDARY as i32;

                coordinate_shifted.div_euclid(WORLD_SIZE as i32)
            });

            let chunk_id = chunk_position_shifted.x
                + chunk_position_shifted.y * WORLD_SIZE as i32
                + chunk_position_shifted.z * WORLD_AREA as i32;

            Some(chunk_id as u32)
        } else {
            None
        }
    }

    pub fn grid_position_to_block_id(grid_position: &IVec3) -> Option<u32> {
        if Simulation::is_on_map(grid_position) {
            let grid_position_shifted = grid_position.map(|coordinate| {
                let coordinate_shifted = coordinate + WORLD_BOUNDARY as i32;

                coordinate_shifted.rem_euclid(CHUNK_SIZE as i32)
            });

            let block_id = grid_position_shifted.x
                + grid_position_shifted.y * CHUNK_SIZE as i32
                + grid_position_shifted.z * CHUNK_AREA as i32;

            Some(block_id as u32)
        } else {
            None
        }
    }

    pub fn get_grid_position(chunk_id: u32, block_id: u32) -> IVec3 {
        let chunk_position = Simulation::chunk_id_to_position(chunk_id);
        let block_position = Simulation::block_id_to_position(block_id);

        let grid_position = CHUNK_SIZE as i32 * chunk_position + block_position;

        grid_position
    }

    // fn grid_to_world(grid_position: &IVec3) -> Vec3 {
    //     grid_position.as_vec3()
    // }

    // fn world_to_grid(world_position: &Vec3) -> IVec3 {
    //     world_position.as_ivec3()
    // }

    fn is_on_map(grid_position: &IVec3) -> bool {
        let in_x_range = grid_position.x.abs() <= WORLD_BOUNDARY as i32;
        let in_y_range = grid_position.y.abs() <= WORLD_BOUNDARY as i32;
        let in_z_range = grid_position.z.abs() <= WORLD_BOUNDARY as i32;

        in_x_range && in_y_range && in_z_range
    }
}
