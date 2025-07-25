//! The simulated environment

pub mod block;
pub mod chunk;
pub mod graph;
pub mod grid;

use crate::simulation::{
    self, constructor,
    consts::*,
    state::{
        physics::aabb::AABB,
        population::{
            entity::{self, Judge},
            nation,
        },
        world::{block::Block, chunk::Chunk, graph::Graph, grid::Grid},
    },
    utils::buffer::Buffer,
};
use glam::{IVec3, Vec3};
use std::{collections::HashMap, sync::RwLock};

pub struct World {
    pub kind: simulation::Kind,
    pub grid: Grid,
    pub block_info_map: HashMap<block::Kind, block::Info>,
    pub chunk_vec: Vec<chunk::Chunk>,
    pub graph_buffer_lock: RwLock<Buffer<Graph>>,
    pub flag_position_map: HashMap<nation::Kind, IVec3>,
}

impl World {
    pub fn new(kind: simulation::Kind) -> Self {
        let grid = Grid::new(kind);
        let block_info_map = block::Info::setup();
        let chunk_vec = Self::setup_chunk_vec(&grid);

        let graph = Graph::new(&grid, 1);
        let graph_buffer_lock = RwLock::new(Buffer::new(graph));

        let flag_position_map = HashMap::from([
            (nation::Kind::Lion, IVec3::ZERO),
            (nation::Kind::Eagle, IVec3::ZERO),
            (nation::Kind::Horse, IVec3::ZERO),
            (nation::Kind::Wolf, IVec3::ZERO),
        ]);

        Self {
            kind,
            grid,
            block_info_map,
            chunk_vec,
            graph_buffer_lock,
            flag_position_map,
        }
    }

    pub fn placeholder() -> Self {
        let kind = simulation::Kind::Placeholder;

        let grid = Grid::new(kind);
        let block_info_map = HashMap::default();
        let chunk_vec = Vec::default();

        let graph = Graph::new(&grid, 1);
        let graph_buffer_lock = RwLock::new(Buffer::new(graph));

        let flag_position_map = HashMap::default();

        Self {
            kind,
            grid,
            block_info_map,
            chunk_vec,
            graph_buffer_lock,
            flag_position_map,
        }
    }

    pub fn get_flag(
        kind: entity::Kind,
        flag_position_map: HashMap<entity::Kind, IVec3>,
    ) -> Option<IVec3> {
        flag_position_map.get(&kind).cloned()
    }

    pub fn setup(kind: simulation::Kind, world: &mut World) {
        match kind {
            simulation::Kind::Main => {
                constructor::world::main::construct(world);

                let graph = Graph::construct(&world.grid, &world.chunk_vec, 1);

                let mut graph_buffer = world.graph_buffer_lock.write().unwrap();
                graph_buffer.update(graph);
            }
            simulation::Kind::Empty => {
                constructor::world::empty::construct(world);
            }
            simulation::Kind::WorldTest => {
                constructor::world::world_test::construct(world);
            }
            simulation::Kind::GraphTest => {
                constructor::world::graph_test::construct(world);

                let graph = Graph::construct(&world.grid, &world.chunk_vec, 1);

                let mut graph_buffer = world.graph_buffer_lock.write().unwrap();
                graph_buffer.update(graph);
            }
            simulation::Kind::Placeholder => (),
        }
    }

    fn setup_chunk_vec(grid: &Grid) -> Vec<chunk::Chunk> {
        Grid::chunk_ids(grid)
            .into_iter()
            .map(|chunk_id| {
                let position = Grid::chunk_id_to_position(grid, chunk_id);

                chunk::Chunk {
                    id: chunk_id,
                    modified: chunk::Modified {
                        block: false,
                        boundary: false,
                    },
                    position,
                    aabb: AABB::new(position.as_vec3(), Vec3::splat(grid.chunk_size as f32)),
                    block_vec: Self::setup_block_vec(grid, chunk_id),
                }
            })
            .collect()
    }

    fn setup_block_vec(grid: &Grid, chunk_id: chunk::ID) -> Vec<block::Block> {
        Grid::block_ids(grid)
            .into_iter()
            .map(|block_id| {
                let position = Grid::ids_to_position(grid, chunk_id, block_id);

                block::Block {
                    id: block_id,
                    chunk_id,
                    position,
                    kind: block::Kind::Empty,
                    solid: false,
                    face_array: Block::face_array(),
                }
            })
            .collect()
    }

    pub fn get_chunk<'a>(
        chunk_id: chunk::ID,
        chunk_vec_slice: &'a [Chunk],
    ) -> Option<&'a chunk::Chunk> {
        chunk_vec_slice.get(usize::from(chunk_id))
    }

    pub fn get_chunk_at<'a>(
        position: IVec3,
        grid: &Grid,
        chunk_vec_slice: &'a [Chunk],
    ) -> Option<&'a chunk::Chunk> {
        let chunk_id = Grid::position_to_chunk_id(grid, position);

        if chunk_id != chunk::ID::MAX {
            chunk_vec_slice.get(usize::from(chunk_id))
        } else {
            None
        }
    }

    pub fn get_chunk_at_mut<'a>(
        position: IVec3,
        grid: &Grid,
        chunk_vec_slice: &'a mut [Chunk],
    ) -> Option<&'a mut chunk::Chunk> {
        let chunk_id = Grid::position_to_chunk_id(grid, position);

        if chunk_id != chunk::ID::MAX {
            chunk_vec_slice.get_mut(usize::from(chunk_id))
        } else {
            None
        }
    }

    pub fn get_block(
        chunk_id: chunk::ID,
        block_id: block::ID,
        chunk_vec_slice: &[Chunk],
    ) -> Option<&block::Block> {
        let chunk = chunk_vec_slice.get(usize::from(chunk_id))?;

        chunk.block_vec.get(usize::from(block_id))
    }

    pub fn get_block_mut(
        chunk_id: chunk::ID,
        block_id: block::ID,
        chunk_vec_slice: &mut [Chunk],
    ) -> Option<&mut block::Block> {
        let chunk = chunk_vec_slice.get_mut(usize::from(chunk_id))?;

        chunk.block_vec.get_mut(usize::from(block_id))
    }

    pub fn get_block_at<'a>(
        position: IVec3,
        grid: &Grid,
        chunk_vec_slice: &'a [Chunk],
    ) -> Option<&'a block::Block> {
        let (chunk_id, block_id) = Grid::position_to_ids(grid, position);

        if chunk_id != chunk::ID::MAX && block_id != block::ID::MAX {
            Self::get_block(chunk_id, block_id, chunk_vec_slice)
        } else {
            None
        }
    }

    pub fn get_block_at_mut<'a>(
        position: IVec3,
        grid: &Grid,
        chunk_vec_slice: &'a mut [Chunk],
    ) -> Option<&'a mut block::Block> {
        let (chunk_id, block_id) = Grid::position_to_ids(grid, position);

        if chunk_id != chunk::ID::MAX && block_id != block::ID::MAX {
            Self::get_block_mut(chunk_id, block_id, chunk_vec_slice)
        } else {
            None
        }
    }

    pub fn get_clearance(position: IVec3, grid: &Grid, chunk_vec_slice: &[Chunk]) -> u32 {
        let ground_is_solid = Self::get_block_at(position + IVec3::NEG_Y, grid, chunk_vec_slice)
            .is_some_and(|block| block.solid);

        let mut clearance = 0;

        if ground_is_solid {
            for level in 0..MAXIMUM_CLEARANCE {
                let level_position = position + IVec3::new(0, level as i32, 0);

                if let Some(block) = Self::get_block_at(level_position, grid, chunk_vec_slice) {
                    if !block.solid {
                        clearance += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        clearance
    }

    fn mark_updates(position1: IVec3, grid: &Grid, chunk_vec_slice: &mut [Chunk]) {
        let mut chunk1_id = None;

        if let Some(chunk1) = Self::get_chunk_at_mut(position1, grid, chunk_vec_slice) {
            chunk1_id = Some(chunk1.id);
            chunk1.modified.block = true;

            if Grid::on_chunk_boundary(grid, position1) {
                chunk1.modified.boundary = true;
            }
        }

        if let Some(chunk1_id) = chunk1_id {
            for direction_offset in grid::Direction::face_offset_array() {
                let position2 = position1 + direction_offset;
                let chunk_id2 = Grid::position_to_chunk_id(grid, position2);

                if chunk_id2 != chunk::ID::MAX && chunk1_id != chunk_id2 {
                    if let Some(chunk2) = chunk_vec_slice.get_mut(usize::from(chunk_id2)) {
                        chunk2.modified.boundary = true;
                    }
                }
            }
        }
    }

    pub fn set_block_kind(
        position: IVec3,
        kind: block::Kind,
        grid: &Grid,
        block_info_map: &HashMap<block::Kind, block::Info>,
        chunk_vec_slice: &mut [Chunk],
    ) -> bool {
        let (chunk_id, block_id) = Grid::position_to_ids(grid, position);

        if chunk_id != chunk::ID::MAX && block_id != block::ID::MAX {
            let block_info = block_info_map.get(&kind).cloned().unwrap();

            if let Some(block) = Self::get_block_mut(chunk_id, block_id, chunk_vec_slice) {
                block.kind = kind;
                block.solid = block_info.solid;
            }

            Self::mark_updates(position, grid, chunk_vec_slice);

            true
        } else {
            log::info!(
                "{:?} block cannot be set at invalid location: {:?}",
                kind,
                position
            );

            false
        }
    }

    pub fn set_box(
        position1: IVec3,
        position2: IVec3,
        kind: block::Kind,
        grid: &Grid,
        block_info_map: &HashMap<block::Kind, block::Info>,
        chunk_vec_slice: &mut [Chunk],
    ) {
        let min = position1.min(position2);
        let max = position1.max(position2);

        for x in min.x..=max.x {
            for y in min.y..=max.y {
                for z in min.z..=max.z {
                    let mut on_boundary = false;

                    if min.x != max.x && (x == min.x || x == max.x) {
                        on_boundary = true;
                    }

                    if min.y != max.y && (y == min.y || y == max.y) {
                        on_boundary = true;
                    }

                    if min.z != max.z && (z == min.z || z == max.z) {
                        on_boundary = true;
                    }

                    let position = IVec3::new(x, y, z);

                    if on_boundary {
                        Self::set_block_kind(position, kind, grid, block_info_map, chunk_vec_slice);
                    } else {
                        Self::set_block_kind(
                            position,
                            block::Kind::Empty,
                            grid,
                            block_info_map,
                            chunk_vec_slice,
                        );
                    }
                }
            }
        }
    }

    pub fn set_cube(
        position1: IVec3,
        position2: IVec3,
        kind: block::Kind,
        grid: &Grid,
        block_info_map: &HashMap<block::Kind, block::Info>,
        chunk_vec_slice: &mut [Chunk],
    ) {
        let min = position1.min(position2);
        let max = position1.max(position2);

        for x in min.x..=max.x {
            for y in min.y..=max.y {
                for z in min.z..=max.z {
                    let position = IVec3::new(x, y, z);

                    Self::set_block_kind(position, kind, grid, block_info_map, chunk_vec_slice);
                }
            }
        }
    }

    pub fn update_chunks(grid: &Grid, chunk_vec_slice: &mut [Chunk]) {
        for chunk_id in Grid::chunk_ids(grid) {
            Self::update_chunk(grid, chunk_id, chunk_vec_slice);
        }
    }

    fn update_chunk(grid: &Grid, chunk_id: chunk::ID, chunk_vec_slice: &mut [Chunk]) {
        if Self::get_chunk(chunk_id, chunk_vec_slice).map_or(false, |chunk| chunk.modified.block) {
            for block_id in Grid::block_ids(grid) {
                let block = Self::get_block(chunk_id, block_id, chunk_vec_slice).unwrap();
                let face_exposure =
                    Self::compute_face_exposure(block.position, grid, chunk_vec_slice);

                if let Some(block) = Self::get_block_mut(chunk_id, block_id, chunk_vec_slice) {
                    Self::update_block_faces(block, face_exposure);
                }
            }
        }
    }

    fn compute_face_exposure(position: IVec3, grid: &Grid, chunk_vec_slice: &[Chunk]) -> [bool; 6] {
        let mut face_exposure = [false; 6];

        for (index, direction) in grid::Direction::face_array().iter().enumerate() {
            let neighbor_pos = position + direction.offset();

            if let Some(neighbor_block) = World::get_block_at(neighbor_pos, grid, chunk_vec_slice) {
                face_exposure[index] = neighbor_block.kind == block::Kind::Empty;
            } else {
                face_exposure[index] = true;
            }
        }

        face_exposure
    }

    fn update_block_faces(block: &mut Block, face_exposure: [bool; 6]) {
        for (face, &exposed) in block.face_array.iter_mut().zip(face_exposure.iter()) {
            face.exposed = exposed;
        }
    }

    pub fn get_visible_chunk_id_vec(
        judge: &Judge,
        grid: &Grid,
        chunk_vec_slice: &[Chunk],
    ) -> Vec<chunk::ID> {
        let mut visible_chunk_id_vec = Vec::new();

        let judge_chunk_coordinates =
            Grid::world_to_chunk_coordinates(grid, judge.spatial.world_position);

        let view_radius = 6;
        let view_direction = judge.spatial.forward();
        let view_origin = judge.eye() + judge.spatial.forward() * -8.0;

        for x in -view_radius..=view_radius {
            for y in -view_radius + 1..=view_radius - 1 {
                for z in -view_radius..=view_radius {
                    let chunk_coordinates = judge_chunk_coordinates + IVec3::new(x, y, z);
                    let chunk_id = Grid::chunk_coordinates_to_chunk_id(grid, chunk_coordinates);

                    if chunk_id != chunk::ID::MAX {
                        if let Some(chunk) = chunk_vec_slice.get(usize::from(chunk_id)) {
                            let origin_to_center = chunk.aabb.center() - view_origin;

                            if view_direction.dot(origin_to_center) >= 0.0 {
                                visible_chunk_id_vec.push(chunk_id);
                            }
                        }
                    }
                }
            }
        }

        visible_chunk_id_vec
    }
}
