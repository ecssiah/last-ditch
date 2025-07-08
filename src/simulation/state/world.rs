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
        population::entity::{self, Judge},
        world::{chunk::Chunk, graph::Graph, grid::Grid},
    },
    utils::buffer::Buffer,
};
use glam::{IVec3, Vec3, Vec4};
use std::{collections::HashMap, sync::RwLock};

pub struct World {
    pub kind: simulation::Kind,
    pub grid: Grid,
    pub block_meta_map: HashMap<block::Kind, block::Meta>,
    pub chunk_vec: Vec<chunk::Chunk>,
    pub graph_buffer_lock: RwLock<Buffer<Graph>>,
    pub flags: HashMap<entity::Kind, IVec3>,
}

impl World {
    pub fn new(kind: simulation::Kind) -> Self {
        let grid = Grid::new(kind);
        let block_meta_map = block::Meta::setup();
        let chunk_vec = Self::setup_chunk_vec(&grid);

        let graph = Graph::new(&grid, 1);
        let graph_buffer_lock = RwLock::new(Buffer::new(graph));

        let flags = HashMap::from([
            (entity::Kind::Lion, IVec3::ZERO),
            (entity::Kind::Eagle, IVec3::ZERO),
            (entity::Kind::Horse, IVec3::ZERO),
            (entity::Kind::Wolf, IVec3::ZERO),
        ]);

        Self {
            kind,
            grid,
            block_meta_map,
            chunk_vec,
            graph_buffer_lock,
            flags,
        }
    }

    pub fn placeholder() -> Self {
        let kind = simulation::Kind::Placeholder;

        let grid = Grid::new(kind);
        let block_meta_map = HashMap::default();
        let chunk_vec = Vec::default();

        let graph = Graph::new(&grid, 1);
        let graph_buffer_lock = RwLock::new(Buffer::new(graph));

        let flags = HashMap::default();

        Self {
            kind,
            grid,
            block_meta_map,
            chunk_vec,
            graph_buffer_lock,
            flags,
        }
    }

    pub fn get_flag(&self, kind: entity::Kind) -> Option<IVec3> {
        self.flags.get(&kind).cloned()
    }

    pub fn setup(&mut self) {
        match self.kind {
            simulation::Kind::Main => {
                constructor::world::main::construct(self);

                let new_graph = Graph::construct(&self.grid, &self.chunk_vec, 1);

                let mut graph_buffer = self.graph_buffer_lock.write().unwrap();
                graph_buffer.update(new_graph);
            }
            simulation::Kind::Empty => {
                constructor::world::empty::construct(self);
            }
            simulation::Kind::WorldTest => {
                constructor::world::world_test::construct(self);
            }
            simulation::Kind::GraphTest => {
                constructor::world::graph_test::construct(self);

                let new_graph = Graph::construct(&self.grid, &self.chunk_vec, 1);

                let mut graph_buffer = self.graph_buffer_lock.write().unwrap();
                graph_buffer.update(new_graph);
            }
            simulation::Kind::Placeholder => (),
        }
    }

    pub fn tick(&mut self) {}

    fn setup_chunk_vec(grid: &Grid) -> Vec<chunk::Chunk> {
        grid.chunk_ids()
            .into_iter()
            .map(|chunk_id| {
                let position = grid.chunk_id_to_position(chunk_id);

                chunk::Chunk {
                    id: chunk_id,
                    modified: chunk::Modified {
                        block: false,
                        boundary: false,
                    },
                    position,
                    aabb: AABB::new(position.as_vec3(), Vec3::splat(grid.chunk_size as f32)),
                    geometry: chunk::Geometry::new(),
                    block_vec: Self::setup_block_vec(grid, chunk_id),
                    visibility_vec: (0..grid.chunk_volume).map(|_| Vec::new()).collect(),
                }
            })
            .collect()
    }

    fn setup_block_vec(grid: &Grid, chunk_id: chunk::ID) -> Vec<block::Block> {
        grid.block_ids()
            .into_iter()
            .map(|block_id| {
                let position = grid.ids_to_position(chunk_id, block_id);

                block::Block {
                    id: block_id,
                    chunk_id,
                    position,
                    kind: block::Kind::Empty,
                    solid: false,
                }
            })
            .collect()
    }

    pub fn get_chunk_at<'a>(
        position: IVec3,
        grid: &Grid,
        chunk_vec_slice: &'a [Chunk],
    ) -> Option<&'a chunk::Chunk> {
        let chunk_id = grid.position_to_chunk_id(position);

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
        let chunk_id = grid.position_to_chunk_id(position);

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

        chunk.get_block(block_id)
    }

    pub fn get_block_mut(
        chunk_id: chunk::ID,
        block_id: block::ID,
        chunk_vec_slice: &mut [Chunk],
    ) -> Option<&mut block::Block> {
        let chunk = chunk_vec_slice.get_mut(usize::from(chunk_id))?;

        chunk.get_block_mut(block_id)
    }

    pub fn get_block_at<'a>(
        position: IVec3,
        grid: &Grid,
        chunk_vec_slice: &'a [Chunk],
    ) -> Option<&'a block::Block> {
        let (chunk_id, block_id) = grid.position_to_ids(position);

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
        let (chunk_id, block_id) = grid.position_to_ids(position);

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

            if grid.on_chunk_boundary(position1) {
                chunk1.modified.boundary = true;
            }
        }

        if let Some(chunk1_id) = chunk1_id {
            for direction in grid::Direction::face_vec() {
                let position2 = position1 + direction.offset();
                let chunk_id2 = grid.position_to_chunk_id(position2);

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
        block_meta_map: &HashMap<block::Kind, block::Meta>,
        chunk_vec_slice: &mut [Chunk],
    ) -> bool {
        let (chunk_id, block_id) = grid.position_to_ids(position);

        if chunk_id != chunk::ID::MAX && block_id != block::ID::MAX {
            let block_meta = block_meta_map.get(&kind).cloned().unwrap();

            if let Some(block) = Self::get_block_mut(chunk_id, block_id, chunk_vec_slice) {
                block.kind = kind;
                block.solid = block_meta.solid;
            }

            Self::update_visibility_vecs(chunk_id, block_id, position, grid, chunk_vec_slice);
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
        block_meta_map: &HashMap<block::Kind, block::Meta>,
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
                        Self::set_block_kind(position, kind, grid, block_meta_map, chunk_vec_slice);
                    } else {
                        Self::set_block_kind(
                            position,
                            block::Kind::Empty,
                            grid,
                            block_meta_map,
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
        block_meta_map: &HashMap<block::Kind, block::Meta>,
        chunk_vec_slice: &mut [Chunk],
    ) {
        let min = position1.min(position2);
        let max = position1.max(position2);

        for x in min.x..=max.x {
            for y in min.y..=max.y {
                for z in min.z..=max.z {
                    let position = IVec3::new(x, y, z);

                    Self::set_block_kind(position, kind, grid, block_meta_map, chunk_vec_slice);
                }
            }
        }
    }

    pub fn update_chunks(grid: &Grid, chunk_vec_slice: &mut [Chunk]) {
        let mut chunk_geometry_update_vec = Vec::new();
        let mut chunk_boundary_update_vec = Vec::new();

        for chunk in chunk_vec_slice.iter_mut() {
            if chunk.modified.block {
                let chunk_geometry = Self::update_chunk_geometry(chunk, grid);

                chunk_geometry_update_vec.push((chunk.id, chunk_geometry));
            }

            if chunk.modified.boundary {
                chunk_boundary_update_vec.push(chunk.id);
            }
        }

        for (chunk_id, geometry) in chunk_geometry_update_vec {
            if let Some(chunk) = chunk_vec_slice.get_mut(usize::from(chunk_id)) {
                chunk.geometry = geometry;

                chunk.modified.block = false;
            }
        }

        for chunk_id in chunk_boundary_update_vec {
            if let Some(chunk) = chunk_vec_slice.get_mut(usize::from(chunk_id)) {
                chunk.modified.boundary = false;
            }
        }
    }

    fn update_chunk_geometry(chunk: &chunk::Chunk, grid: &Grid) -> chunk::Geometry {
        let mut chunk_geometry = chunk::Geometry::new();

        for block_id in grid.block_ids() {
            let block = chunk.get_block(block_id).unwrap();

            if block.solid {
                let visibility_vec = &chunk.visibility_vec[usize::from(block_id)];
                let position = grid.ids_to_position(chunk.id, block_id);

                for direction in grid::Direction::face_vec() {
                    if visibility_vec.contains(&direction) {
                        let mut face = block::Face::new(position, direction, block.kind);

                        let (edges, corners) = Self::get_face_neighbors(direction, visibility_vec);

                        face.light = Self::calculate_face_light(edges, corners);

                        chunk_geometry.face_vec.push(face);
                    }
                }
            }
        }

        chunk_geometry
    }

    fn update_visibility_vecs(
        chunk_id: chunk::ID,
        block_id: block::ID,
        position: IVec3,
        grid: &Grid,
        chunk_vec_slice: &mut [Chunk],
    ) {
        let mut visibility_updates_map: HashMap<chunk::ID, Vec<(block::ID, Vec<grid::Direction>)>> =
            HashMap::new();

        if Self::get_block(chunk_id, block_id, chunk_vec_slice)
            .is_some_and(|block| block.kind != block::Kind::Empty)
        {
            let visibility_vec = Self::compute_visibility_vec(position, grid, chunk_vec_slice);

            visibility_updates_map
                .entry(chunk_id)
                .or_default()
                .push((block_id, visibility_vec));
        }

        for offset in grid::Direction::face_offsets() {
            let neighbor_position = position + offset;

            let (chunk_id, block_id) = grid.position_to_ids(neighbor_position);

            if chunk_id != chunk::ID::MAX && block_id != block::ID::MAX {
                if Self::get_block(chunk_id, block_id, chunk_vec_slice)
                    .is_some_and(|block| block.kind != block::Kind::Empty)
                {
                    let visibility_vec =
                        Self::compute_visibility_vec(neighbor_position, grid, chunk_vec_slice);

                    visibility_updates_map
                        .entry(chunk_id)
                        .or_default()
                        .push((block_id, visibility_vec));
                }
            }
        }

        for (chunk_id, visibility_update_vec) in visibility_updates_map {
            if let Some(chunk) = chunk_vec_slice.get_mut(usize::from(chunk_id)) {
                for (block_id, visibility_vec) in visibility_update_vec {
                    chunk.visibility_vec[usize::from(block_id)] = visibility_vec;
                }
            }
        }
    }

    fn compute_visibility_vec(
        position: IVec3,
        grid: &Grid,
        chunk_vec_slice: &[Chunk],
    ) -> Vec<grid::Direction> {
        grid::Direction::face_vec()
            .iter()
            .filter_map(|&direction| {
                let neighbor_position = position + direction.offset();
                let block = Self::get_block_at(neighbor_position, grid, chunk_vec_slice);

                block
                    .filter(|block| block.kind == block::Kind::Empty)
                    .map(|_| direction)
            })
            .collect()
    }

    fn get_face_neighbors(
        direction: grid::Direction,
        visibility_vec: &[grid::Direction],
    ) -> ([bool; 4], [bool; 4]) {
        match direction {
            grid::Direction::XpYoZo => (
                [
                    visibility_vec.contains(&grid::Direction::XpYnZo),
                    visibility_vec.contains(&grid::Direction::XpYoZn),
                    visibility_vec.contains(&grid::Direction::XpYpZo),
                    visibility_vec.contains(&grid::Direction::XpYoZp),
                ],
                [
                    visibility_vec.contains(&grid::Direction::XpYnZn),
                    visibility_vec.contains(&grid::Direction::XpYpZn),
                    visibility_vec.contains(&grid::Direction::XpYpZp),
                    visibility_vec.contains(&grid::Direction::XpYnZp),
                ],
            ),
            grid::Direction::XnYoZo => (
                [
                    visibility_vec.contains(&grid::Direction::XnYnZo),
                    visibility_vec.contains(&grid::Direction::XnYoZp),
                    visibility_vec.contains(&grid::Direction::XnYpZo),
                    visibility_vec.contains(&grid::Direction::XnYoZn),
                ],
                [
                    visibility_vec.contains(&grid::Direction::XnYnZp),
                    visibility_vec.contains(&grid::Direction::XnYpZp),
                    visibility_vec.contains(&grid::Direction::XnYpZn),
                    visibility_vec.contains(&grid::Direction::XnYnZn),
                ],
            ),
            grid::Direction::XoYpZo => (
                [
                    visibility_vec.contains(&grid::Direction::XoYpZn),
                    visibility_vec.contains(&grid::Direction::XnYpZo),
                    visibility_vec.contains(&grid::Direction::XoYpZp),
                    visibility_vec.contains(&grid::Direction::XpYpZo),
                ],
                [
                    visibility_vec.contains(&grid::Direction::XnYpZn),
                    visibility_vec.contains(&grid::Direction::XnYpZp),
                    visibility_vec.contains(&grid::Direction::XpYpZp),
                    visibility_vec.contains(&grid::Direction::XpYpZn),
                ],
            ),
            grid::Direction::XoYnZo => (
                [
                    visibility_vec.contains(&grid::Direction::XoYnZn),
                    visibility_vec.contains(&grid::Direction::XpYnZo),
                    visibility_vec.contains(&grid::Direction::XoYnZp),
                    visibility_vec.contains(&grid::Direction::XnYnZo),
                ],
                [
                    visibility_vec.contains(&grid::Direction::XpYnZn),
                    visibility_vec.contains(&grid::Direction::XpYnZp),
                    visibility_vec.contains(&grid::Direction::XnYnZp),
                    visibility_vec.contains(&grid::Direction::XnYnZn),
                ],
            ),
            grid::Direction::XoYoZp => (
                [
                    visibility_vec.contains(&grid::Direction::XoYnZp),
                    visibility_vec.contains(&grid::Direction::XpYoZp),
                    visibility_vec.contains(&grid::Direction::XoYpZp),
                    visibility_vec.contains(&grid::Direction::XnYoZp),
                ],
                [
                    visibility_vec.contains(&grid::Direction::XpYnZp),
                    visibility_vec.contains(&grid::Direction::XpYpZp),
                    visibility_vec.contains(&grid::Direction::XnYpZp),
                    visibility_vec.contains(&grid::Direction::XnYnZp),
                ],
            ),
            grid::Direction::XoYoZn => (
                [
                    visibility_vec.contains(&grid::Direction::XoYnZn),
                    visibility_vec.contains(&grid::Direction::XnYoZn),
                    visibility_vec.contains(&grid::Direction::XoYpZn),
                    visibility_vec.contains(&grid::Direction::XpYoZn),
                ],
                [
                    visibility_vec.contains(&grid::Direction::XnYnZn),
                    visibility_vec.contains(&grid::Direction::XnYpZn),
                    visibility_vec.contains(&grid::Direction::XpYpZn),
                    visibility_vec.contains(&grid::Direction::XpYnZn),
                ],
            ),
            _ => panic!("Invalid Direction: {:?}", direction),
        }
    }

    fn calculate_face_light(edges: [bool; 4], corners: [bool; 4]) -> Vec4 {
        Vec4::new(
            Self::calculate_vertex_light(edges[3], edges[0], corners[3]),
            Self::calculate_vertex_light(edges[0], edges[1], corners[0]),
            Self::calculate_vertex_light(edges[1], edges[2], corners[1]),
            Self::calculate_vertex_light(edges[2], edges[3], corners[2]),
        )
    }

    fn calculate_vertex_light(edge1: bool, edge2: bool, corner: bool) -> f32 {
        if edge1 && edge2 {
            AMBIENT_LIGHT_LEVELS[0]
        } else if corner || edge1 || edge2 {
            AMBIENT_LIGHT_LEVELS[1]
        } else {
            AMBIENT_LIGHT_LEVELS[2]
        }
    }

    pub fn get_visible_chunk_id_vec(
        judge: &Judge,
        grid: &Grid,
        chunk_vec_slice: &[Chunk],
    ) -> Vec<chunk::ID> {
        let mut visible_chunk_id_vec = Vec::new();

        let judge_chunk_coordinates = grid.world_to_chunk_coordinates(judge.spatial.world_position);

        let view_radius = 6;
        let view_direction = judge.spatial.forward();
        let view_origin = judge.eye() + judge.spatial.forward() * -8.0;

        for x in -view_radius..=view_radius {
            for y in -view_radius + 1..=view_radius - 1 {
                for z in -view_radius..=view_radius {
                    let chunk_coordinates = judge_chunk_coordinates + IVec3::new(x, y, z);
                    let chunk_id = grid.chunk_coordinates_to_chunk_id(chunk_coordinates);

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
