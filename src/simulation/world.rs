//! The Simulation module contains all of the logic required to generate and evolve
//! the core civilizational garden.

pub mod block;
pub mod chunk;
pub mod grid;

use crate::simulation::{
    consts::*,
    population::agent::{self},
    time::Tick,
    BLOCK_MAP,
};
use glam::{IVec3, Vec4};
use std::collections::HashMap;

pub struct World {
    pub tick: Tick,
    pub grid: grid::Grid,
    pub chunk_list: Vec<chunk::Chunk>,
    pub flags: HashMap<agent::Kind, IVec3>,
}

impl World {
    pub fn new(radius: u32, chunk_radius: u32) -> World {
        let tick = Tick::ZERO;
        let grid = grid::Grid::new(radius, chunk_radius);
        let chunk_list = Self::setup_chunks(&grid);

        let flags = HashMap::from([
            (agent::Kind::Lion, IVec3::ZERO),
            (agent::Kind::Eagle, IVec3::ZERO),
            (agent::Kind::Horse, IVec3::ZERO),
            (agent::Kind::Wolf, IVec3::ZERO),
        ]);

        let world = Self {
            tick,
            grid,
            chunk_list,
            flags,
        };

        world
    }

    pub fn get_flag(&self, kind: agent::Kind) -> Option<IVec3> {
        self.flags.get(&kind).cloned()
    }

    fn block_ids(&self) -> Vec<block::ID> {
        (0..self.grid.chunk_volume)
            .map(|index| block::ID(index))
            .collect()
    }

    pub fn setup(&mut self) {
        if TESTING {
            self.setup_test_world();
        } else {
            self.setup_main_world();
        }
    }

    pub fn setup_main_world(&mut self) {
        log::info!("Setup Ground");

        self.setup_ground();

        log::info!("Setup Structures");

        self.setup_compass();

        self.setup_temple(0, 0, 34, agent::Kind::Eagle);
        self.setup_temple(-34, 0, 0, agent::Kind::Lion);
        self.setup_temple(0, 0, -34, agent::Kind::Horse);
        self.setup_temple(34, 0, 0, agent::Kind::Wolf);

        self.setup_observation_deck();

        self.update_chunks();
    }

    pub fn setup_test_world(&mut self) {
        let chunk_radius = self.grid.chunk_radius as i32;
        // let chunk_size = self.grid.chunk_size as i32;

        let boundary = self.grid.boundary as i32;

        self.set_cube(
            IVec3::new(-boundary, -boundary, -boundary),
            IVec3::new(boundary, boundary, boundary),
            block::Kind::Polished1,
        );

        self.set_block_kind(0, -chunk_radius, 1, block::Kind::North);
        self.set_block_kind(-1, -chunk_radius, 0, block::Kind::West);
        self.set_block_kind(0, -chunk_radius, -1, block::Kind::South);
        self.set_block_kind(1, -chunk_radius, 0, block::Kind::East);

        let chunk_center_grid_position = self.grid.chunk_to_grid(IVec3::new(0, 0, 0)).unwrap();
        let chunk_north_grid_position = self.grid.chunk_to_grid(IVec3::new(0, 0, 1)).unwrap();
        // let chunk_south_grid_position = self.grid.chunk_to_grid(IVec3::new(0, 0, -1)).unwrap();
        // let chunk_east_grid_position = self.grid.chunk_to_grid(IVec3::new(1, 0, 0)).unwrap();
        let chunk_west_grid_position = self.grid.chunk_to_grid(IVec3::new(-1, 0, 0)).unwrap();
        // let chunk_up_grid_position = self.grid.chunk_to_grid(IVec3::new(0, 1, 0)).unwrap();
        // let chunk_down_grid_position = self.grid.chunk_to_grid(IVec3::new(0, -1, 0)).unwrap();

        self.set_cube(
            chunk_center_grid_position
                + IVec3::new(-chunk_radius + 1, -chunk_radius + 1, -chunk_radius + 1),
            chunk_center_grid_position
                + IVec3::new(chunk_radius - 1, chunk_radius - 1, chunk_radius - 1),
            block::Kind::Empty,
        );

        self.set_cube(
            chunk_center_grid_position + IVec3::new(0, -chunk_radius + 1, 0),
            chunk_west_grid_position + IVec3::new(0, -chunk_radius + 3, 0),
            block::Kind::Empty,
        );

        self.set_cube(
            chunk_west_grid_position
                + IVec3::new(-chunk_radius + 1, -chunk_radius + 1, -chunk_radius + 1),
            chunk_west_grid_position
                + IVec3::new(chunk_radius - 1, chunk_radius - 1, chunk_radius - 1),
            block::Kind::Empty,
        );

        self.set_cube(
            chunk_center_grid_position + IVec3::new(0, -chunk_radius + 1, 0),
            chunk_north_grid_position + IVec3::new(0, -chunk_radius + 3, 0),
            block::Kind::Empty,
        );

        self.set_cube(
            chunk_north_grid_position
                + IVec3::new(-chunk_radius + 1, -chunk_radius + 1, -chunk_radius + 1),
            chunk_north_grid_position
                + IVec3::new(chunk_radius - 1, chunk_radius - 1, chunk_radius - 1),
            block::Kind::Empty,
        );

        self.set_block_kind(
            chunk_north_grid_position.x - 2,
            chunk_north_grid_position.y - chunk_radius + 1,
            chunk_north_grid_position.z + 2,
            block::Kind::Origin,
        );

        self.set_block_kind(
            chunk_north_grid_position.x - 1,
            chunk_north_grid_position.y - chunk_radius + 2,
            chunk_north_grid_position.z + 2,
            block::Kind::Origin,
        );

        self.set_block_kind(
            chunk_north_grid_position.x,
            chunk_north_grid_position.y - chunk_radius + 3,
            chunk_north_grid_position.z + 2,
            block::Kind::Origin,
        );

        self.set_block_kind(
            chunk_north_grid_position.x + 1,
            chunk_north_grid_position.y - chunk_radius + 4,
            chunk_north_grid_position.z + 2,
            block::Kind::Origin,
        );

        self.set_block_kind(
            chunk_north_grid_position.x + 2,
            chunk_north_grid_position.y - chunk_radius + 5,
            chunk_north_grid_position.z + 2,
            block::Kind::Origin,
        );

        self.update_chunks();
    }

    pub fn tick(&mut self, tick: &Tick) {
        self.tick = *tick;
    }

    fn mark_update(&mut self, chunk_id: chunk::ID) {
        if let Some(chunk) = self.get_chunk_mut(chunk_id) {
            chunk.updated = true;
        }
    }

    pub fn update_chunks(&mut self) {
        let mut chunk_updates = Vec::new();

        for chunk in &self.chunk_list {
            if chunk.updated {
                let graph = self.update_chunk_graph(chunk);
                let geometry = self.update_chunk_geometry(chunk);

                chunk_updates.push((chunk.id, graph, geometry));
            }
        }

        for (chunk_id, graph, geometry) in chunk_updates {
            if let Some(chunk) = self.get_chunk_mut(chunk_id) {
                chunk.graph = graph;
                chunk.geometry = geometry;

                chunk.updated = false;
            }
        }
    }

    fn setup_chunks(grid: &grid::Grid) -> Vec<chunk::Chunk> {
        let chunk_list = (0..grid.volume)
            .map(|index| {
                let chunk_id = chunk::ID(index);
                let chunk_position = grid.chunk_id_to_position(chunk_id).unwrap();

                chunk::Chunk {
                    id: chunk_id,
                    tick: Tick::ZERO,
                    updated: false,
                    position: chunk_position,
                    graph: chunk::Graph::new(),
                    geometry: chunk::Geometry::new(),
                    kind_list: Vec::from([block::Kind::Empty]),
                    block_list: (0..grid.chunk_volume).map(|_| 0).collect(),
                    visibility_list: (0..grid.chunk_volume).map(|_| Vec::new()).collect(),
                }
            })
            .collect();

        chunk_list
    }

    fn setup_ground(&mut self) {
        let ground_boundary = (self.grid.boundary - self.grid.chunk_size) as isize;

        for x in -ground_boundary..=ground_boundary {
            for y in -1..=0 {
                for z in -ground_boundary..=ground_boundary {
                    let grid_position = IVec3::new(x as i32, y as i32, z as i32);
                    let chunk_position = self.grid.grid_to_chunk(grid_position).unwrap();

                    let kind = if (chunk_position.x + chunk_position.y + chunk_position.z) % 2 == 0
                    {
                        block::Kind::Polished1
                    } else {
                        block::Kind::Polished2
                    };

                    self.set_block_kind(grid_position.x, grid_position.y, grid_position.z, kind);
                }
            }
        }
    }

    fn setup_compass(&mut self) {
        self.set_block_kind(0, 0, 0, block::Kind::Origin);
        self.set_block_kind(0, 0, 4, block::Kind::North);
        self.set_block_kind(-4, 0, 0, block::Kind::West);
        self.set_block_kind(0, 0, -4, block::Kind::South);
        self.set_block_kind(4, 0, 0, block::Kind::East);
    }

    fn setup_temple(&mut self, x: i32, y: i32, z: i32, kind: agent::Kind) {
        self.flags.insert(kind, IVec3::new(x, y + 2, z));

        self.set_block_kind(x, y + 6, z, kind.icon());

        self.set_cube(
            IVec3::new(x - 8, y + 1, z - 8),
            IVec3::new(x + 8, y + 1, z + 8),
            block::Kind::Stone1,
        );

        self.set_cube(
            IVec3::new(x - 7, y + 2, z - 7),
            IVec3::new(x + 7, y + 2, z + 7),
            block::Kind::Stone1,
        );

        self.set_cube(
            IVec3::new(x - 6, y + 8, z - 6),
            IVec3::new(x + 6, y + 8, z + 6),
            block::Kind::Stone1,
        );

        self.set_cube(
            IVec3::new(x - 5, y + 9, z - 5),
            IVec3::new(x + 5, y + 9, z + 5),
            block::Kind::Stone1,
        );

        self.set_cube(
            IVec3::new(x - 5, y + 8, z - 5),
            IVec3::new(x + 5, y + 8, z + 5),
            block::Kind::Empty,
        );

        self.set_cube(
            IVec3::new(x + 5, y + 1, z + 5),
            IVec3::new(x + 5, y + 8, z + 5),
            block::Kind::Engraved1,
        );

        self.set_cube(
            IVec3::new(x - 5, y + 1, z + 5),
            IVec3::new(x - 5, y + 8, z + 5),
            block::Kind::Engraved1,
        );

        self.set_cube(
            IVec3::new(x + 5, y + 1, z - 5),
            IVec3::new(x + 5, y + 8, z - 5),
            block::Kind::Engraved1,
        );

        self.set_cube(
            IVec3::new(x - 5, y + 1, z - 5),
            IVec3::new(x - 5, y + 8, z - 5),
            block::Kind::Engraved1,
        );
    }

    fn setup_observation_deck(&mut self) {
        let inner_offset = 17 * 3 - 4;
        let outer_offset = 17 * 3 + 3;
        let height = 16;

        self.set_cube(
            IVec3::new(-outer_offset, height, -outer_offset),
            IVec3::new(outer_offset, height, outer_offset),
            block::Kind::Polished1,
        );

        self.set_cube(
            IVec3::new(-inner_offset, height, -inner_offset),
            IVec3::new(inner_offset, height, inner_offset),
            block::Kind::Empty,
        );
    }

    pub fn get_chunk(&self, chunk_id: chunk::ID) -> Option<&chunk::Chunk> {
        let chunk = self.chunk_list.get(usize::from(chunk_id))?;

        Some(chunk)
    }

    pub fn get_chunk_mut(&mut self, chunk_id: chunk::ID) -> Option<&mut chunk::Chunk> {
        let chunk = self.chunk_list.get_mut(usize::from(chunk_id))?;

        Some(chunk)
    }

    pub fn get_chunk_at(&self, grid_position: IVec3) -> Option<&chunk::Chunk> {
        let chunk_id = self.grid.grid_to_chunk_id(grid_position)?;

        let chunk = self.get_chunk(chunk_id);

        chunk
    }

    pub fn get_block(&self, chunk_id: chunk::ID, block_id: block::ID) -> Option<&block::Block> {
        let chunk = self.get_chunk(chunk_id)?;

        let kind_id = *chunk.block_list.get(usize::from(block_id))?;
        let kind = chunk.kind_list.get(usize::from(kind_id))?;

        let block = BLOCK_MAP.get(&kind)?;

        Some(block)
    }

    pub fn get_block_at(&self, grid_position: IVec3) -> Option<&block::Block> {
        let (chunk_id, block_id) = self.grid.grid_to_ids(grid_position)?;

        self.get_block(chunk_id, block_id)
    }

    pub fn set_block_kind(&mut self, x: i32, y: i32, z: i32, kind: block::Kind) {
        let grid_position = IVec3::new(x, y, z);

        if let Some((chunk_id, block_id)) = self.grid.grid_to_ids(grid_position) {
            self.update_kind_list(chunk_id, block_id, kind);
            self.update_visibility_lists(chunk_id, block_id, grid_position);

            self.mark_update(chunk_id);
        }
    }

    pub fn set_cube(&mut self, point1: IVec3, point2: IVec3, kind: block::Kind) {
        let min = point1.min(point2);
        let max = point1.max(point2);

        for x in min.x..=max.x {
            for y in min.y..=max.y {
                for z in min.z..=max.z {
                    self.set_block_kind(x, y, z, kind);
                }
            }
        }
    }

    fn update_kind_list(&mut self, chunk_id: chunk::ID, block_id: block::ID, kind: block::Kind) {
        if let Some(chunk) = self.get_chunk_mut(chunk_id) {
            let kind_id = match Self::get_kind_id(&chunk, kind) {
                Some(kind_id) => kind_id,
                None => {
                    chunk.kind_list.push(kind);
                    chunk.kind_list.len() - 1
                }
            };

            chunk.block_list[usize::from(block_id)] = kind_id;
        }
    }

    fn get_kind_id(chunk: &chunk::Chunk, kind: block::Kind) -> Option<usize> {
        let kind_id = chunk
            .kind_list
            .iter()
            .position(|target_kind| kind == *target_kind)?;

        Some(kind_id)
    }

    fn update_visibility_lists(
        &mut self,
        chunk_id: chunk::ID,
        block_id: block::ID,
        grid_position: IVec3,
    ) {
        let mut visibility_updates_map: HashMap<chunk::ID, Vec<(block::ID, Vec<grid::Direction>)>> =
            HashMap::new();

        if let Some(block) = self.get_block(chunk_id, block_id) {
            if block.kind != block::Kind::Empty {
                let visibility_list = self.compute_visibility_list(grid_position);

                visibility_updates_map
                    .entry(chunk_id)
                    .or_insert_with(Vec::new)
                    .push((block_id, visibility_list));
            }
        }

        for offset in grid::Direction::face_offsets() {
            let neighbor_grid_position = grid_position + offset;

            if let Some((chunk_id, block_id)) = self.grid.grid_to_ids(neighbor_grid_position) {
                if let Some(block) = self.get_block(chunk_id, block_id) {
                    if block.kind != block::Kind::Empty {
                        let visibility_list = self.compute_visibility_list(neighbor_grid_position);

                        visibility_updates_map
                            .entry(chunk_id)
                            .or_insert_with(Vec::new)
                            .push((block_id, visibility_list));
                    }
                }
            }
        }

        for (chunk_id, visibility_update_list) in visibility_updates_map {
            if let Some(chunk) = self.get_chunk_mut(chunk_id) {
                for (block_id, visibility_list) in visibility_update_list {
                    chunk.visibility_list[usize::from(block_id)] = visibility_list;
                }
            }
        }
    }

    fn compute_visibility_list(&self, grid_position: IVec3) -> Vec<grid::Direction> {
        let visibility_list: Vec<grid::Direction> = grid::Direction::faces()
            .iter()
            .filter_map(|&direction| {
                let neighbor_grid_position = grid_position + direction.offset();
                let block = self.get_block_at(neighbor_grid_position);

                block
                    .filter(|block| block.kind == block::Kind::Empty)
                    .map(|_| direction)
            })
            .collect();

        visibility_list
    }

    fn update_chunk_graph(&self, chunk: &chunk::Chunk) -> chunk::Graph {
        let mut node_list = Vec::<chunk::Node>::new();
        let mut position_index_map = HashMap::<IVec3, usize>::new();

        for block_id in self.block_ids() {
            if let Some(block) = self.get_block(chunk.id, block_id) {
                if block.solid {
                    if let Some(grid_position) = self.grid.ids_to_grid(chunk.id, block_id) {
                        let index = node_list.len();
                        let clearance = self.get_clearance(grid_position);

                        if clearance > 0 {
                            let node = chunk::Node {
                                grid_position,
                                clearance,
                            };

                            position_index_map.insert(node.grid_position, index);
                            node_list.push(node);
                        }
                    }
                }
            }
        }

        let mut edge_list = Vec::<chunk::Edge>::new();

        for (index, node) in node_list.iter().enumerate() {
            for neighbor_direction in grid::Direction::neighbors() {
                if neighbor_direction == grid::Direction::XoYpZo {
                    continue;
                }

                if neighbor_direction == grid::Direction::XoYnZo {
                    continue;
                }

                let offset = neighbor_direction.offset();
                let neighbor_grid_position = node.grid_position + offset;

                if let Some(&neighbor_index) = position_index_map.get(&neighbor_grid_position) {
                    let edge = chunk::Edge {
                        source: index,
                        target: neighbor_index,
                        cost: neighbor_direction.cost(),
                    };

                    edge_list.push(edge);
                }
            }
        }

        let connection_list = Vec::<chunk::Connection>::new();

        chunk::Graph {
            node_list,
            edge_list,
            connection_list,
        }
    }

    fn update_chunk_geometry(&self, chunk: &chunk::Chunk) -> chunk::Geometry {
        let mut face_list = Vec::new();

        for block_id in self.block_ids() {
            let block = self.get_block(chunk.id, block_id).unwrap();

            if block.solid {
                let visibility_list = &chunk.visibility_list[usize::from(block_id)];
                let grid_position = self.grid.ids_to_grid(chunk.id, block_id).unwrap();

                for direction in grid::Direction::faces() {
                    if visibility_list.contains(&direction) {
                        let mut face = block::Face::new(grid_position, direction, block.kind);

                        let (face_edges, face_corners) =
                            Self::get_face_neighbors(direction, &visibility_list);

                        face.light = Self::calculate_face_light(face_edges, face_corners);

                        face_list.push(face);
                    }
                }
            }
        }

        chunk::Geometry { face_list }
    }

    fn get_face_neighbors(
        direction: grid::Direction,
        visibility_list: &Vec<grid::Direction>,
    ) -> ([bool; 4], [bool; 4]) {
        match direction {
            grid::Direction::XpYoZo => (
                [
                    visibility_list.contains(&grid::Direction::XpYnZo),
                    visibility_list.contains(&grid::Direction::XpYoZn),
                    visibility_list.contains(&grid::Direction::XpYpZo),
                    visibility_list.contains(&grid::Direction::XpYoZp),
                ],
                [
                    visibility_list.contains(&grid::Direction::XpYnZn),
                    visibility_list.contains(&grid::Direction::XpYpZn),
                    visibility_list.contains(&grid::Direction::XpYpZp),
                    visibility_list.contains(&grid::Direction::XpYnZp),
                ],
            ),
            grid::Direction::XnYoZo => (
                [
                    visibility_list.contains(&grid::Direction::XnYnZo),
                    visibility_list.contains(&grid::Direction::XnYoZp),
                    visibility_list.contains(&grid::Direction::XnYpZo),
                    visibility_list.contains(&grid::Direction::XnYoZn),
                ],
                [
                    visibility_list.contains(&grid::Direction::XnYnZp),
                    visibility_list.contains(&grid::Direction::XnYpZp),
                    visibility_list.contains(&grid::Direction::XnYpZn),
                    visibility_list.contains(&grid::Direction::XnYnZn),
                ],
            ),
            grid::Direction::XoYpZo => (
                [
                    visibility_list.contains(&grid::Direction::XoYpZn),
                    visibility_list.contains(&grid::Direction::XnYpZo),
                    visibility_list.contains(&grid::Direction::XoYpZp),
                    visibility_list.contains(&grid::Direction::XpYpZo),
                ],
                [
                    visibility_list.contains(&grid::Direction::XnYpZn),
                    visibility_list.contains(&grid::Direction::XnYpZp),
                    visibility_list.contains(&grid::Direction::XpYpZp),
                    visibility_list.contains(&grid::Direction::XpYpZn),
                ],
            ),
            grid::Direction::XoYnZo => (
                [
                    visibility_list.contains(&grid::Direction::XoYnZn),
                    visibility_list.contains(&grid::Direction::XpYnZo),
                    visibility_list.contains(&grid::Direction::XoYnZp),
                    visibility_list.contains(&grid::Direction::XnYnZo),
                ],
                [
                    visibility_list.contains(&grid::Direction::XpYnZn),
                    visibility_list.contains(&grid::Direction::XpYnZp),
                    visibility_list.contains(&grid::Direction::XnYnZp),
                    visibility_list.contains(&grid::Direction::XnYnZn),
                ],
            ),
            grid::Direction::XoYoZp => (
                [
                    visibility_list.contains(&grid::Direction::XoYnZp),
                    visibility_list.contains(&grid::Direction::XpYoZp),
                    visibility_list.contains(&grid::Direction::XoYpZp),
                    visibility_list.contains(&grid::Direction::XnYoZp),
                ],
                [
                    visibility_list.contains(&grid::Direction::XpYnZp),
                    visibility_list.contains(&grid::Direction::XpYpZp),
                    visibility_list.contains(&grid::Direction::XnYpZp),
                    visibility_list.contains(&grid::Direction::XnYnZp),
                ],
            ),
            grid::Direction::XoYoZn => (
                [
                    visibility_list.contains(&grid::Direction::XoYnZn),
                    visibility_list.contains(&grid::Direction::XnYoZn),
                    visibility_list.contains(&grid::Direction::XoYpZn),
                    visibility_list.contains(&grid::Direction::XpYoZn),
                ],
                [
                    visibility_list.contains(&grid::Direction::XnYnZn),
                    visibility_list.contains(&grid::Direction::XnYpZn),
                    visibility_list.contains(&grid::Direction::XpYpZn),
                    visibility_list.contains(&grid::Direction::XpYnZn),
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

    pub fn has_clearance(&self, grid_position: IVec3, height: i32) -> bool {
        let base_is_solid = self
            .get_block_at(grid_position)
            .map(|block| block.solid)
            .unwrap_or(false);

        let clear_above = (1..=height).all(|level| {
            let vertical_grid_position = grid_position + level * IVec3::Y;

            self.get_block_at(vertical_grid_position)
                .map(|block| !block.solid)
                .unwrap_or(false)
        });

        base_is_solid && clear_above
    }

    pub fn get_clearance(&self, grid_position: IVec3) -> usize {
        let base_is_solid = self
            .get_block_at(grid_position)
            .map(|block| block.solid)
            .unwrap_or(false);

        if base_is_solid {
            let mut clearance = 0;

            for level in 1..=MAXIMUM_CLEARANCE_CHECK {
                let vertical_grid_position = grid_position + IVec3::Y * level;

                let is_clear = self
                    .get_block_at(vertical_grid_position)
                    .map(|block| !block.solid)
                    .unwrap_or(false);

                if is_clear {
                    clearance += 1;
                } else {
                    break;
                }
            }

            clearance
        } else {
            0
        }
    }

    pub fn get_visible_chunk_id_list(&self, chunk_id: chunk::ID) -> Vec<chunk::ID> {
        let radius = JUDGE_VIEW_RADIUS as i32;
        let chunk_count_estimate = ((2 * radius + 1).pow(3)) as usize;

        let mut visible_chunk_id_list = Vec::with_capacity(chunk_count_estimate);

        if let Some(chunk_position) = self.grid.chunk_id_to_position(chunk_id) {
            for dx in -radius..=radius {
                for dy in -radius..=radius {
                    for dz in -radius..=radius {
                        let offset = dx.abs() + dy.abs() + dz.abs();

                        if offset > radius {
                            continue;
                        }

                        let visible_chunk_position = chunk_position + IVec3::new(dx, dy, dz);

                        if let Some(grid_position) = self.grid.chunk_to_grid(visible_chunk_position)
                        {
                            if let Some(visible_chunk_id) =
                                self.grid.grid_to_chunk_id(grid_position)
                            {
                                visible_chunk_id_list.push(visible_chunk_id);
                            }
                        }
                    }
                }
            }
        }

        visible_chunk_id_list
    }
}
