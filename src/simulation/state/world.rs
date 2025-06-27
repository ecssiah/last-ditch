//! The simulated environment

pub mod block;
pub mod builder;
pub mod chunk;
pub mod graph;
pub mod grid;

use crate::simulation::{
    self,
    consts::*,
    state::{
        physics::aabb::AABB,
        population::entity::{self, Judge},
        world::{self, graph::Graph, grid::Grid},
    },
};
use glam::{IVec3, Vec3, Vec4};
use std::collections::HashMap;

pub struct World {
    pub mode: simulation::Mode,
    pub grid: Grid,
    pub block_meta_map: HashMap<block::Kind, block::Meta>,
    pub chunk_vec: Vec<chunk::Chunk>,
    pub graph: Graph,
    pub flags: HashMap<entity::Kind, IVec3>,
}

impl World {
    pub fn new(mode: simulation::Mode) -> Self {
        let grid = Grid::new(mode);
        let block_meta_map = block::Meta::setup();
        let chunk_vec = Self::setup_chunk_vec(&grid);
        let graph = Graph::new(&grid, 1);

        let flags = HashMap::from([
            (entity::Kind::Lion, IVec3::ZERO),
            (entity::Kind::Eagle, IVec3::ZERO),
            (entity::Kind::Horse, IVec3::ZERO),
            (entity::Kind::Wolf, IVec3::ZERO),
        ]);

        Self {
            mode,
            grid,
            block_meta_map,
            chunk_vec,
            graph,
            flags,
        }
    }

    pub fn get_flag(&self, kind: entity::Kind) -> Option<IVec3> {
        self.flags.get(&kind).cloned()
    }

    pub fn setup(&mut self) {
        match self.mode {
            simulation::Mode::Main => world::builder::main::construct(self),
            simulation::Mode::WorldTest => world::builder::world_test::construct(self),
            simulation::Mode::GraphTest => world::builder::graph_test::construct(self),
        }

        self.graph.setup(&self.chunk_vec);
    }

    pub fn tick(&mut self) {}

    fn setup_chunk_vec(grid: &Grid) -> Vec<chunk::Chunk> {
        grid.chunk_ids()
            .into_iter()
            .map(|chunk_id| {
                let position = grid.chunk_id_to_position(chunk_id).unwrap();

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
                let position = grid.ids_to_position(chunk_id, block_id).unwrap();

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

    pub fn get_chunk(&self, chunk_id: chunk::ID) -> Option<&chunk::Chunk> {
        self.chunk_vec.get(usize::from(chunk_id))
    }

    pub fn get_chunk_mut(&mut self, chunk_id: chunk::ID) -> Option<&mut chunk::Chunk> {
        self.chunk_vec.get_mut(usize::from(chunk_id))
    }

    pub fn get_chunk_at(&self, position: IVec3) -> Option<&chunk::Chunk> {
        let chunk_id = self.grid.position_to_chunk_id(position)?;

        self.get_chunk(chunk_id)
    }

    pub fn get_block(&self, chunk_id: chunk::ID, block_id: block::ID) -> Option<&block::Block> {
        let chunk = self.get_chunk(chunk_id)?;

        chunk.get_block(block_id)
    }

    pub fn get_block_mut(
        &mut self,
        chunk_id: chunk::ID,
        block_id: block::ID,
    ) -> Option<&mut block::Block> {
        let chunk = self.get_chunk_mut(chunk_id)?;

        chunk.get_block_mut(block_id)
    }

    pub fn get_block_at(&self, position: IVec3) -> Option<&block::Block> {
        let (chunk_id, block_id) = self.grid.position_to_ids(position)?;

        self.get_block(chunk_id, block_id)
    }

    pub fn get_block_at_mut(&mut self, position: IVec3) -> Option<&mut block::Block> {
        let (chunk_id, block_id) = self.grid.position_to_ids(position)?;

        self.get_block_mut(chunk_id, block_id)
    }

    pub fn get_clearance(&self, position: IVec3) -> Option<u32> {
        let ground_is_solid = self
            .get_block_at(position + IVec3::NEG_Y)
            .is_some_and(|block| block.solid);

        if ground_is_solid {
            let mut clearance = 0;

            for level in 0..MAXIMUM_CLEARANCE {
                let level_position = position + IVec3::new(0, level as i32, 0);

                if let Some(block) = self.get_block_at(level_position) {
                    if !block.solid {
                        clearance += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            Some(clearance)
        } else {
            None
        }
    }

    fn mark_updates(&mut self, chunk_id1: chunk::ID, position1: IVec3) {
        self.set_block_modified(chunk_id1, true);

        if self.grid.on_chunk_boundary(position1) {
            self.set_boundary_modified(chunk_id1, true);

            for direction in grid::Direction::face_vec() {
                let position2 = position1 + direction.offset();

                if let Some(chunk_id2) = self
                    .grid
                    .position_to_chunk_id(position2)
                    .filter(|chunk_id| chunk_id != &chunk_id1)
                {
                    self.set_boundary_modified(chunk_id2, true);
                }
            }
        }
    }

    pub fn set_block_kind(&mut self, position: IVec3, kind: block::Kind) {
        if let Some((chunk_id, block_id)) = self.grid.position_to_ids(position) {
            let block_meta = self.block_meta_map.get(&kind).cloned().unwrap();

            if let Some(block) = self.get_block_mut(chunk_id, block_id) {
                block.kind = kind;
                block.solid = block_meta.solid;
            }

            self.update_visibility_vecs(chunk_id, block_id, position);
            self.mark_updates(chunk_id, position);
        } else {
            log::info!(
                "{:?} block not set at invalid location: {:?}",
                kind,
                position
            );
        }
    }

    pub fn set_box(&mut self, position1: IVec3, position2: IVec3, kind: block::Kind) {
        let min = position1.min(position2);
        let max = position1.max(position2);

        for x in min.x..=max.x {
            for y in min.y..=max.y {
                for z in min.z..=max.z {
                    let on_boundary = x == min.x
                        || x == max.x
                        || y == min.y
                        || y == max.y
                        || z == min.z
                        || z == max.z;

                    if on_boundary {
                        let position = IVec3::new(x, y, z);

                        self.set_block_kind(position, kind);
                    }
                }
            }
        }
    }

    pub fn set_cube(&mut self, position1: IVec3, position2: IVec3, kind: block::Kind) {
        let min = position1.min(position2);
        let max = position1.max(position2);

        for x in min.x..=max.x {
            for y in min.y..=max.y {
                for z in min.z..=max.z {
                    let position = IVec3::new(x, y, z);

                    self.set_block_kind(position, kind);
                }
            }
        }
    }

    fn set_block_modified(&mut self, chunk_id: chunk::ID, modified: bool) {
        if let Some(chunk) = self.get_chunk_mut(chunk_id) {
            chunk.modified.block = modified;
        }
    }

    fn set_boundary_modified(&mut self, chunk_id: chunk::ID, modified: bool) {
        if let Some(chunk) = self.get_chunk_mut(chunk_id) {
            chunk.modified.boundary = modified;
        }
    }

    pub fn update_chunks(&mut self) {
        let mut chunk_geometry_update_vec = Vec::new();
        let mut chunk_boundary_update_vec = Vec::new();

        for chunk in &self.chunk_vec {
            if chunk.modified.block {
                let chunk_geometry = self.update_chunk_geometry(chunk);

                chunk_geometry_update_vec.push((chunk.id, chunk_geometry));
            }

            if chunk.modified.boundary {
                chunk_boundary_update_vec.push(chunk.id);
            }
        }

        for (chunk_id, geometry) in chunk_geometry_update_vec {
            if let Some(chunk) = self.get_chunk_mut(chunk_id) {
                chunk.geometry = geometry;

                chunk.modified.block = false;
            }
        }

        for chunk_id in chunk_boundary_update_vec {
            self.set_boundary_modified(chunk_id, false);
        }
    }

    fn update_chunk_geometry(&self, chunk: &chunk::Chunk) -> chunk::Geometry {
        let mut chunk_geometry = chunk::Geometry::new();

        for block_id in self.grid.block_ids() {
            let block = self.get_block(chunk.id, block_id).unwrap();

            if block.solid {
                let visibility_vec = &chunk.visibility_vec[usize::from(block_id)];
                let position = self.grid.ids_to_position(chunk.id, block_id).unwrap();

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
        &mut self,
        chunk_id: chunk::ID,
        block_id: block::ID,
        position: IVec3,
    ) {
        let mut visibility_updates_map: HashMap<chunk::ID, Vec<(block::ID, Vec<grid::Direction>)>> =
            HashMap::new();

        if self
            .get_block(chunk_id, block_id)
            .is_some_and(|block| block.kind != block::Kind::Empty)
        {
            let visibility_vec = self.compute_visibility_vec(position);

            visibility_updates_map
                .entry(chunk_id)
                .or_default()
                .push((block_id, visibility_vec));
        }

        for offset in grid::Direction::face_offsets() {
            let neighbor_position = position + offset;

            if let Some((chunk_id, block_id)) = self.grid.position_to_ids(neighbor_position) {
                if self
                    .get_block(chunk_id, block_id)
                    .is_some_and(|block| block.kind != block::Kind::Empty)
                {
                    let visibility_vec = self.compute_visibility_vec(neighbor_position);

                    visibility_updates_map
                        .entry(chunk_id)
                        .or_default()
                        .push((block_id, visibility_vec));
                }
            }
        }

        for (chunk_id, visibility_update_vec) in visibility_updates_map {
            if let Some(chunk) = self.get_chunk_mut(chunk_id) {
                for (block_id, visibility_vec) in visibility_update_vec {
                    chunk.visibility_vec[usize::from(block_id)] = visibility_vec;
                }
            }
        }
    }

    fn compute_visibility_vec(&self, position: IVec3) -> Vec<grid::Direction> {
        grid::Direction::face_vec()
            .iter()
            .filter_map(|&direction| {
                let neighbor_position = position + direction.offset();
                let block = self.get_block_at(neighbor_position);

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

    pub fn get_visible_chunk_id_vec(&self, judge: &Judge) -> Vec<chunk::ID> {
        let mut visible_chunk_id_vec = Vec::new();

        let judge_chunk_coordinates = self
            .grid
            .world_to_chunk_coordinates(judge.spatial.world_position)
            .unwrap();

        let view_radius = 6;
        let view_direction = judge.spatial.forward();
        let view_origin = judge.eye() + judge.spatial.forward() * -8.0;

        for x in -view_radius..=view_radius {
            for y in -view_radius + 1..=view_radius - 1 {
                for z in -view_radius..=view_radius {
                    let chunk_coordinates = judge_chunk_coordinates + IVec3::new(x, y, z);

                    if let Some(chunk_id) =
                        self.grid.chunk_coordinates_to_chunk_id(chunk_coordinates)
                    {
                        if let Some(chunk) = self.get_chunk(chunk_id) {
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
