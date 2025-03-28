//! The Simulation module contains all of the logic required to generate and evolve
//! the core civilizational garden.

pub mod action;
pub mod agent;
pub mod block;
pub mod chunk;
pub mod consts;
pub mod id;
pub mod physics;
pub mod state;
pub mod structure;
pub mod world;

use crate::{
    interface::consts::DEBUG_COLOR,
    simulation::action::{JumpAction, MovementAction},
};
use action::{Action, AgentAction, WorldAction};
use agent::Agent;
pub use block::Block;
use block::{BlockID, Direction, Face, Neighbors};
pub use chunk::Chunk;
use chunk::ChunkID;
pub use consts::*;
use glam::{IVec3, Quat, Vec3, Vec4};
use physics::Physics;
use state::State;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    thread,
    time::{Duration, Instant},
};
use tokio::sync::mpsc::UnboundedReceiver;
use world::World;

pub struct Simulation {
    action_rx: UnboundedReceiver<Action>,
    state: Arc<State>,
}

impl Simulation {
    pub fn new(action_rx: UnboundedReceiver<Action>) -> Self {
        let state = Arc::from(State {
            agent: Self::setup_agent(),
            world: Self::setup_world(),
            physics: Self::setup_physics(),
            chunks: Self::setup_chunks(),
        });

        let simulation = Self { action_rx, state };

        log::info!("Simulation Initialized");

        simulation
    }

    pub fn run(&mut self) {
        let mut accumulator = 0.0;
        let mut previous = Instant::now();

        loop {
            let now = Instant::now();
            let frame_time = now.duration_since(previous).as_secs_f64();
            previous = now;

            accumulator += frame_time;

            while accumulator >= FIXED_DT {
                self.update();
                accumulator -= FIXED_DT;
            }

            thread::sleep(Duration::from_micros(SIMULATION_WAIT_DURATION));
        }
    }

    fn setup_agent() -> Arc<RwLock<Agent>> {
        let mut agent = Agent::new();

        agent.set_position(3.0, 3.0, 3.0);
        agent.set_rotation(0.0, 0.0);

        Arc::from(RwLock::from(agent))
    }

    fn setup_world() -> Arc<RwLock<World>> {
        let world = World {
            active: true,
            seed: SEED,
            time: 0.0,
            ticks: 1,
            last_update: 1,
        };

        Arc::from(RwLock::from(world))
    }

    fn setup_physics() -> Arc<RwLock<Physics>> {
        let physics = Physics::new();

        Arc::from(RwLock::from(physics))
    }

    fn setup_chunks() -> Arc<[Arc<RwLock<Chunk>>]> {
        let chunks: [Arc<RwLock<Chunk>>; WORLD_VOLUME] = core::array::from_fn(|chunk_id| {
            Arc::from(RwLock::from(Chunk {
                last_update: 1,
                id: chunk_id,
                position: Self::chunk_id_to_position(chunk_id),
                palette: Vec::from([block::Kind::Air]),
                palette_ids: Box::new([0; CHUNK_VOLUME]),
                meta: Box::new([block::Meta::default(); CHUNK_VOLUME]),
                light: Box::new([block::LightLevel::default(); CHUNK_VOLUME]),
                mesh: chunk::Mesh::default(),
            }))
        });

        Arc::from(chunks)
    }

    pub fn generate(&mut self) {
        self.generate_ground();

        self.set_block_kind(0, 2, 0, block::Kind::GoldMetal);
        self.set_block_kind(1, 1, 0, block::Kind::GoldMetal);
        self.set_block_kind(-1, 1, 0, block::Kind::GoldMetal);
        self.set_block_kind(0, 1, 1, block::Kind::GoldMetal);
        self.set_block_kind(0, 1, -1, block::Kind::GoldMetal);

        self.generate_structure(0, 0, -20, structure::Kind::Luigi);
        self.generate_structure(-20, 0, 0, structure::Kind::Mario);
        self.generate_structure(20, 0, 0, structure::Kind::Mario);
        self.generate_structure(0, 0, 20, structure::Kind::Luigi);

        self.set_block_kind(0, 48, 0, block::Kind::Metal);
        self.set_block_kind(-1, 48, 0, block::Kind::Metal);
        self.set_block_kind(1, 48, 0, block::Kind::Metal);
        self.set_block_kind(0, 48, 1, block::Kind::Metal);
        self.set_block_kind(0, 48, -1, block::Kind::Metal);

        let agent = self.state.agent.read().unwrap();
        let mut physics = self.state.physics.write().unwrap();

        physics.add_agent(&agent);
    }

    fn generate_structure(&mut self, x: i32, y: i32, z: i32, structure_kind: structure::Kind) {
        if let Some(structure) = STRUCTURES.get(&structure_kind) {
            let world_position = IVec3::new(x, y, z);

            for block_data in &structure.blocks[..] {
                let block_position = IVec3::new(
                    block_data.position[0],
                    block_data.position[1],
                    block_data.position[2],
                );

                let grid_position = world_position + block_position;

                self.set_block_kind(
                    grid_position.x,
                    grid_position.y,
                    grid_position.z,
                    block_data.kind,
                );
            }
        }
    }

    fn generate_ground(&mut self) {
        for x in -(CHUNK_RADIUS as isize)..=(CHUNK_RADIUS as isize) {
            for z in -(CHUNK_RADIUS as isize)..=(CHUNK_RADIUS as isize) {
                let kind = if (x % 2 == 0) ^ (z % 2 == 0) {
                    block::Kind::White
                } else {
                    block::Kind::Grey
                };

                self.set_block_kind(x as i32, 0, z as i32, kind);
            }
        }
    }

    pub fn get_chunk(&self, grid_position: IVec3) -> Option<Arc<RwLock<chunk::Chunk>>> {
        let chunk_id = Self::grid_position_to_chunk_id(grid_position)?;

        Some(Arc::clone(&self.state.chunks[chunk_id]))
    }

    pub fn get_block(&self, grid_position: IVec3) -> Option<&block::Block> {
        let (chunk_id, block_id) = Self::grid_position_to_ids(grid_position)?;

        let chunk = self.state.chunks[chunk_id].write().unwrap();

        let palette_id = chunk.palette_ids[block_id];
        let kind = chunk.palette[palette_id];

        let block = BLOCKS.get(&kind)?;

        Some(block)
    }

    pub fn get_meta(&self, grid_position: IVec3) -> Option<block::Meta> {
        let (chunk_id, block_id) = Self::grid_position_to_ids(grid_position)?;

        let chunk = self.state.chunks[chunk_id].write().unwrap();

        let meta = chunk.meta[block_id];

        Some(meta)
    }

    fn set_block_kind(&mut self, x: i32, y: i32, z: i32, kind: block::Kind) {
        let grid_position = IVec3::new(x, y, z);

        if let Some((chunk_id, block_id)) = Self::grid_position_to_ids(grid_position) {
            self.update_palette(chunk_id, block_id, kind);

            self.update_neighbors(grid_position);
            self.update_visibility(grid_position);
            self.update_light(chunk_id, block_id, grid_position);
            self.update_chunk_mesh(chunk_id);

            self.set_chunk_last_update(chunk_id);
        }
    }

    fn update_palette(&mut self, chunk_id: ChunkID, block_id: BlockID, kind: block::Kind) {
        let mut chunk = self.state.chunks[chunk_id].write().unwrap();

        let palette_id = self.get_and_insert_palette_id(&mut chunk, kind);
        chunk.palette_ids[block_id] = palette_id;
    }

    fn update_neighbors(&mut self, grid_position: IVec3) {
        let mut updates: HashMap<ChunkID, Vec<(BlockID, Neighbors)>> = HashMap::new();

        for offset in Direction::offsets() {
            let neighbor_grid_position = grid_position + offset;

            if let Some((chunk_id, block_id)) = Self::grid_position_to_ids(neighbor_grid_position) {
                let neighbors = self.compute_neighbors(neighbor_grid_position);

                updates
                    .entry(chunk_id)
                    .or_insert_with(Vec::new)
                    .push((block_id, neighbors));
            }
        }

        for (chunk_id, updates) in updates.iter() {
            let mut chunk = self.state.chunks[*chunk_id].write().unwrap();

            for (block_id, neighbors) in updates {
                chunk.meta[*block_id].neighbors = *neighbors;
            }
        }
    }

    fn compute_neighbors(&mut self, grid_position: IVec3) -> Neighbors {
        let mut neighbors = Neighbors::NONE;

        for index in 0..Direction::offsets().len() {
            if index == Direction::X0_Y0_Z0.index() {
                continue;
            }

            if let Some(offset) = Direction::get_offset(index) {
                let neighbor_grid_position = grid_position + offset;

                if let Some(block) = self.get_block(neighbor_grid_position) {
                    if block.solid {
                        if let Some(direction) = Direction::bit(index) {
                            neighbors.set_solid(direction, true);
                        }
                    }
                }
            }
        }

        neighbors
    }

    fn update_visibility(&mut self, grid_position: IVec3) {
        let mut updates: HashMap<ChunkID, Vec<(BlockID, Face)>> = HashMap::new();

        if let Some((chunk_id, block_id)) = Self::grid_position_to_ids(grid_position) {
            let visibility = self.compute_visibility(grid_position);

            updates
                .entry(chunk_id)
                .or_insert_with(Vec::new)
                .push((block_id, visibility));
        }

        for offset in Direction::face_offsets() {
            let neighbor_grid_position = grid_position + offset;

            if let Some((chunk_id, block_id)) = Self::grid_position_to_ids(neighbor_grid_position) {
                if let Some(block) = self.get_block(neighbor_grid_position) {
                    if block.kind != block::Kind::Air {
                        let visibility = self.compute_visibility(neighbor_grid_position);

                        updates
                            .entry(chunk_id)
                            .or_insert_with(Vec::new)
                            .push((block_id, visibility));
                    }
                }
            }
        }

        for (chunk_id, updates) in updates.iter() {
            let mut chunk = self.state.chunks[*chunk_id].write().unwrap();

            for (block_id, visibility) in updates {
                chunk.meta[*block_id].visibility = *visibility;
            }
        }
    }

    fn compute_visibility(&self, grid_position: IVec3) -> Face {
        let mut visibility = Face::empty();

        for index in 0..Direction::offsets().len() {
            if index == Direction::X0_Y0_Z0.index() {
                continue;
            }

            if let Some(offset) = Direction::get_offset(index) {
                let neighbor_grid_position = grid_position + offset;

                if let Some(block) = self.get_block(neighbor_grid_position) {
                    if block.kind == block::Kind::Air {
                        if let Some(direction) = Direction::bit(index) {
                            match direction {
                                Direction::XP_Y0_Z0 => visibility.insert(Face::XP),
                                Direction::XN_Y0_Z0 => visibility.insert(Face::XN),
                                Direction::X0_YP_Z0 => visibility.insert(Face::YP),
                                Direction::X0_YN_Z0 => visibility.insert(Face::YN),
                                Direction::X0_Y0_ZP => visibility.insert(Face::ZP),
                                Direction::X0_Y0_ZN => visibility.insert(Face::ZN),
                                _ => (),
                            }
                        }
                    }
                }
            }
        }

        visibility
    }

    fn update_light(&mut self, _chunk_id: ChunkID, _block_id: BlockID, _grid_position: IVec3) {}

    fn update_chunk_mesh(&mut self, chunk_id: ChunkID) {
        let mesh = self.generate_chunk_mesh(chunk_id);

        {
            let mut chunk = self.state.chunks[chunk_id].write().unwrap();
            chunk.mesh = mesh;
        }

        let chunk = &self.state.chunks[chunk_id].read().unwrap();

        if chunk.mesh.vertices.len() > 0 {
            let mut physics = self.state.physics.write().unwrap();

            physics.add_chunk_collider(chunk_id, &chunk.mesh.vertices, &chunk.mesh.indices);
        }
    }

    fn generate_chunk_mesh(&self, chunk_id: ChunkID) -> chunk::Mesh {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let chunk = self.state.chunks[chunk_id].read().unwrap();

        for block_id in 0..CHUNK_VOLUME {
            let grid_position = Simulation::ids_to_grid_position(chunk_id, block_id);

            let meta = &chunk.meta[block_id];
            let block = chunk.get_block(block_id).unwrap();

            for face in block::Face::ALL {
                if meta.visibility.contains(face) == false {
                    continue;
                }

                let face_quad = Self::generate_quad(grid_position, face);
                let normal = face.normal();

                let color = if DEBUG_COLOR {
                    face.debug_color()
                } else {
                    Vec4::new(block.color.0, block.color.1, block.color.2, block.color.3)
                };

                let face_ao = Self::calculate_ao(meta.neighbors, face);

                let chunk_vertices = face_quad.iter().enumerate().map(|(index, position)| {
                    let position = *position;
                    let ao = face_ao[index];

                    chunk::Vertex {
                        position,
                        normal,
                        color,
                        ao,
                    }
                });

                let start_index = vertices.len() as u32;

                let face_indices = [
                    start_index + 0,
                    start_index + 1,
                    start_index + 2,
                    start_index + 0,
                    start_index + 2,
                    start_index + 3,
                ];

                vertices.extend(chunk_vertices);
                indices.extend(face_indices);
            }
        }

        chunk::Mesh { vertices, indices }
    }

    fn generate_quad(grid_position: IVec3, face: block::Face) -> [Vec3; 4] {
        let base = grid_position.as_vec3();
        let offsets = face.quad();

        offsets.map(|offset| base + offset)
    }

    fn calculate_ao(neighbors: block::Neighbors, face: block::Face) -> [f32; 4] {
        match face {
            block::Face::XP => Self::calculate_face_ao(
                [
                    neighbors.is_solid(block::Direction::XP_YN_Z0),
                    neighbors.is_solid(block::Direction::XP_Y0_ZN),
                    neighbors.is_solid(block::Direction::XP_YP_Z0),
                    neighbors.is_solid(block::Direction::XP_Y0_ZP),
                ],
                [
                    neighbors.is_solid(block::Direction::XP_YN_ZN),
                    neighbors.is_solid(block::Direction::XP_YP_ZN),
                    neighbors.is_solid(block::Direction::XP_YP_ZP),
                    neighbors.is_solid(block::Direction::XP_YN_ZP),
                ],
            ),
            block::Face::XN => Self::calculate_face_ao(
                [
                    neighbors.is_solid(block::Direction::XN_YN_Z0),
                    neighbors.is_solid(block::Direction::XN_Y0_ZP),
                    neighbors.is_solid(block::Direction::XN_YP_Z0),
                    neighbors.is_solid(block::Direction::XN_Y0_ZN),
                ],
                [
                    neighbors.is_solid(block::Direction::XN_YN_ZP),
                    neighbors.is_solid(block::Direction::XN_YP_ZP),
                    neighbors.is_solid(block::Direction::XN_YP_ZN),
                    neighbors.is_solid(block::Direction::XN_YN_ZN),
                ],
            ),
            block::Face::YP => Self::calculate_face_ao(
                [
                    neighbors.is_solid(block::Direction::X0_YP_ZN),
                    neighbors.is_solid(block::Direction::XN_YP_Z0),
                    neighbors.is_solid(block::Direction::X0_YP_ZP),
                    neighbors.is_solid(block::Direction::XP_YP_Z0),
                ],
                [
                    neighbors.is_solid(block::Direction::XN_YP_ZN),
                    neighbors.is_solid(block::Direction::XN_YP_ZP),
                    neighbors.is_solid(block::Direction::XP_YP_ZP),
                    neighbors.is_solid(block::Direction::XP_YP_ZN),
                ],
            ),
            block::Face::YN => Self::calculate_face_ao(
                [
                    neighbors.is_solid(block::Direction::X0_YN_ZN),
                    neighbors.is_solid(block::Direction::XP_YN_Z0),
                    neighbors.is_solid(block::Direction::X0_YN_ZP),
                    neighbors.is_solid(block::Direction::XN_YN_Z0),
                ],
                [
                    neighbors.is_solid(block::Direction::XP_YN_ZN),
                    neighbors.is_solid(block::Direction::XP_YN_ZP),
                    neighbors.is_solid(block::Direction::XN_YN_ZP),
                    neighbors.is_solid(block::Direction::XN_YN_ZN),
                ],
            ),
            block::Face::ZP => Self::calculate_face_ao(
                [
                    neighbors.is_solid(block::Direction::X0_YN_ZP),
                    neighbors.is_solid(block::Direction::XP_Y0_ZP),
                    neighbors.is_solid(block::Direction::X0_YP_ZP),
                    neighbors.is_solid(block::Direction::XN_Y0_ZP),
                ],
                [
                    neighbors.is_solid(block::Direction::XP_YN_ZP),
                    neighbors.is_solid(block::Direction::XP_YP_ZP),
                    neighbors.is_solid(block::Direction::XN_YP_ZP),
                    neighbors.is_solid(block::Direction::XN_YN_ZP),
                ],
            ),
            block::Face::ZN => Self::calculate_face_ao(
                [
                    neighbors.is_solid(block::Direction::X0_YN_ZN),
                    neighbors.is_solid(block::Direction::XN_Y0_ZN),
                    neighbors.is_solid(block::Direction::X0_YP_ZN),
                    neighbors.is_solid(block::Direction::XP_Y0_ZN),
                ],
                [
                    neighbors.is_solid(block::Direction::XN_YN_ZN),
                    neighbors.is_solid(block::Direction::XN_YP_ZN),
                    neighbors.is_solid(block::Direction::XP_YP_ZN),
                    neighbors.is_solid(block::Direction::XP_YN_ZN),
                ],
            ),
            _ => panic!("Invalid Face: {:?}", face),
        }
    }

    fn calculate_face_ao(edges: [bool; 4], corners: [bool; 4]) -> [f32; 4] {
        [
            Self::calculate_vertex_ao(edges[3], edges[0], corners[3]),
            Self::calculate_vertex_ao(edges[0], edges[1], corners[0]),
            Self::calculate_vertex_ao(edges[1], edges[2], corners[1]),
            Self::calculate_vertex_ao(edges[2], edges[3], corners[2]),
        ]
    }

    fn calculate_vertex_ao(edge1: bool, edge2: bool, corner: bool) -> f32 {
        if edge1 && edge2 {
            AMBIENT_OCCLUSION_LEVEL[2]
        } else if corner || edge1 || edge2 {
            AMBIENT_OCCLUSION_LEVEL[1]
        } else {
            AMBIENT_OCCLUSION_LEVEL[0]
        }
    }

    fn set_chunk_last_update(&mut self, chunk_id: usize) {
        let mut world = self.state.world.write().unwrap();
        let mut chunk = self.state.chunks[chunk_id].write().unwrap();

        chunk.last_update = world.ticks;
        world.last_update = world.ticks;
    }

    fn get_and_insert_palette_id(&self, chunk: &mut Chunk, kind: block::Kind) -> usize {
        match chunk
            .palette
            .iter()
            .position(|palette_kind| kind == *palette_kind)
        {
            Some(id) => id,
            None => {
                chunk.palette.push(kind.clone());

                let id = chunk.palette.len() - 1;
                id
            }
        }
    }

    fn update(&mut self) {
        self.handle_actions();
        self.evolve_world();

        {
            let mut physics = self.state.physics.write().unwrap();
            physics.update_agent_movement(self.state.agent.clone());
            physics.update_agent_jump(self.state.agent.clone());

            physics.step();
        }

        {
            let physics = self.state.physics.write().unwrap();
            let mut agent = self.state.agent.write().unwrap();

            physics.sync_agent_transforms(&mut *agent);
        }
    }

    pub fn get_state(&self) -> Arc<State> {
        Arc::clone(&self.state)
    }

    fn handle_actions(&mut self) {
        while let Ok(action) = self.action_rx.try_recv() {
            match action {
                Action::World(WorldAction::Quit) => {
                    self.handle_quit_action();
                }
                Action::Agent(AgentAction::Movement(movement_actions)) => {
                    self.handle_movement_action(&movement_actions);
                }
                Action::Agent(AgentAction::Jump(jump_action)) => {
                    self.handle_jump_action(&jump_action);
                }
            }
        }
    }

    fn handle_quit_action(&mut self) {
        let mut world = self.state.world.write().unwrap();

        world.active = false;
    }

    fn handle_movement_action(&mut self, movement_actions: &MovementAction) {
        let mut agent = self.state.agent.write().unwrap();

        agent.z_speed = movement_actions.direction.z;
        agent.x_speed = movement_actions.direction.x;

        if movement_actions.rotation.length_squared() > 1e-6 {
            agent.look_x_axis -= movement_actions.rotation.x;
            agent.look_y_axis += movement_actions.rotation.y;

            let limit = 89.0_f32.to_radians();

            agent.look_x_axis = agent.look_x_axis.clamp(-limit, limit);

            let y_axis_quat = Quat::from_rotation_y(agent.look_y_axis);
            let x_axis_quat = Quat::from_rotation_x(agent.look_x_axis);

            let target_rotation = y_axis_quat * x_axis_quat;

            agent.look_rotation = agent.look_rotation.slerp(target_rotation, 0.3);
        }
    }

    fn handle_jump_action(&mut self, jump_action: &JumpAction) {
        match jump_action {
            JumpAction::Start => {
                {
                    let mut agent = self.state.agent.write().unwrap();

                    agent.jump_state.active = true;
                    agent.jump_state.timer = 0.0;
                    agent.jump_state.cancel = false;
                }

                let mut physics = self.state.physics.write().unwrap();
                physics.begin_agent_jump(self.state.agent.clone());
            }
            JumpAction::End => {
                let mut agent = self.state.agent.write().unwrap();

                agent.jump_state.cancel = true;
            }
        }
    }

    fn evolve_world(&mut self) {
        let mut state = self.state.world.write().unwrap();

        state.time += FIXED_DT;
        state.ticks += 1;
    }

    pub fn chunk_id_to_position(chunk_id: ChunkID) -> IVec3 {
        let chunk_position_shifted = IVec3::new(
            (chunk_id % WORLD_SIZE) as i32,
            (chunk_id / WORLD_SIZE % WORLD_SIZE) as i32,
            (chunk_id / WORLD_AREA) as i32,
        );

        let chunk_position = chunk_position_shifted - IVec3::splat(WORLD_RADIUS as i32);

        chunk_position
    }

    pub fn chunk_id_to_world_position(chunk_id: ChunkID) -> Vec3 {
        let grid_position = Self::chunk_id_to_position(chunk_id);
        let world_position = grid_position.as_vec3() * CHUNK_SIZE as f32;

        world_position
    }

    pub fn block_id_to_position(block_id: BlockID) -> IVec3 {
        let block_position_shifted = IVec3::new(
            (block_id % CHUNK_SIZE) as i32,
            (block_id / CHUNK_SIZE % CHUNK_SIZE) as i32,
            (block_id / CHUNK_AREA) as i32,
        );

        let block_position = block_position_shifted - IVec3::splat(CHUNK_RADIUS as i32);

        block_position
    }

    pub fn grid_position_to_chunk_id(grid_position: IVec3) -> Option<ChunkID> {
        if Self::on_map(grid_position) {
            let chunk_position_shifted = grid_position.map(|coordinate| {
                let coordinate_shifted = coordinate + WORLD_BOUNDARY as i32;

                coordinate_shifted.div_euclid(WORLD_SIZE as i32)
            });

            let chunk_id = chunk_position_shifted.x
                + chunk_position_shifted.y * WORLD_SIZE as i32
                + chunk_position_shifted.z * WORLD_AREA as i32;

            Some(chunk_id as ChunkID)
        } else {
            None
        }
    }

    pub fn grid_position_to_block_id(grid_position: IVec3) -> Option<BlockID> {
        if Self::on_map(grid_position) {
            let grid_position_shifted = grid_position.map(|coordinate| {
                let coordinate_shifted = coordinate + WORLD_BOUNDARY as i32;

                coordinate_shifted.rem_euclid(CHUNK_SIZE as i32)
            });

            let block_id = grid_position_shifted.x
                + grid_position_shifted.y * CHUNK_SIZE as i32
                + grid_position_shifted.z * CHUNK_AREA as i32;

            Some(block_id as BlockID)
        } else {
            None
        }
    }

    pub fn grid_position_to_ids(grid_position: IVec3) -> Option<(ChunkID, BlockID)> {
        let chunk_id = Simulation::grid_position_to_chunk_id(grid_position)?;
        let block_id = Simulation::grid_position_to_block_id(grid_position)?;

        Some((chunk_id, block_id))
    }

    pub fn ids_to_grid_position(chunk_id: ChunkID, block_id: BlockID) -> IVec3 {
        let chunk_position = Self::chunk_id_to_position(chunk_id);
        let block_position = Self::block_id_to_position(block_id);

        let grid_position = CHUNK_SIZE as i32 * chunk_position + block_position;

        grid_position
    }

    pub fn on_map(grid_position: IVec3) -> bool {
        let in_x_range = grid_position.x.abs() <= WORLD_BOUNDARY as i32;
        let in_y_range = grid_position.y.abs() <= WORLD_BOUNDARY as i32;
        let in_z_range = grid_position.z.abs() <= WORLD_BOUNDARY as i32;

        in_x_range && in_y_range && in_z_range
    }
}
