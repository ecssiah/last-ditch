//! The simulated environment

pub mod block;
pub mod builder;
pub mod chunk;
pub mod graph;
pub mod grid;

pub use graph::Graph;
pub use grid::Grid;

use crate::simulation::{
    consts::*,
    population::{
        agent::{self},
        Judge,
    },
    time::Tick,
    world,
};
use glam::{IVec3, Vec4};
use std::collections::{HashMap, HashSet, VecDeque};

pub struct World {
    pub tick: Tick,
    pub grid: grid::Grid,
    pub block_meta_map: HashMap<block::Kind, block::Meta>,
    pub chunk_list: Vec<chunk::Chunk>,
    pub graph: world::Graph,
    pub flags: HashMap<agent::Kind, IVec3>,
}

impl World {
    pub fn new(chunk_radius: u32, world_radius: u32) -> World {
        let tick = Tick::ZERO;
        let grid = grid::Grid::new(chunk_radius, world_radius);
        let block_meta_map = block::Meta::setup();
        let chunk_list = Self::setup_chunk_list(&grid);
        let graph = Self::setup_world_graph(&chunk_list);

        let flags = HashMap::from([
            (agent::Kind::Lion, IVec3::ZERO),
            (agent::Kind::Eagle, IVec3::ZERO),
            (agent::Kind::Horse, IVec3::ZERO),
            (agent::Kind::Wolf, IVec3::ZERO),
        ]);

        let world = Self {
            tick,
            grid,
            block_meta_map,
            chunk_list,
            graph,
            flags,
        };

        world
    }

    pub fn get_flag(&self, kind: agent::Kind) -> Option<IVec3> {
        self.flags.get(&kind).cloned()
    }

    pub fn setup(&mut self) {
        if TESTING {
            builder::TestWorld::build(self)
        } else {
            builder::MainWorld::build(self);
        }
    }

    pub fn tick(&mut self, tick: &Tick) {
        self.tick = *tick;
    }

    fn setup_world_graph(chunk_list: &Vec<chunk::Chunk>) -> world::Graph {
        let mut graph = world::Graph::new();

        for chunk in chunk_list {
            let chunk_node = chunk::Node {
                chunk_id: chunk.id,
                position: chunk.position,
                group_id: 0,
            };

            let chunk_graph = chunk::Graph::new();

            graph.add_chunk_node(chunk.id, chunk_node);
            graph.add_chunk_graph(chunk.id, chunk_graph);
        }

        graph
    }

    fn setup_chunk_list(grid: &grid::Grid) -> Vec<chunk::Chunk> {
        grid.chunk_ids()
            .into_iter()
            .map(|chunk_id| chunk::Chunk {
                id: chunk_id,
                tick: Tick::ZERO,
                modified: chunk::Modified {
                    block: false,
                    boundary: false,
                },
                position: grid.chunk_id_to_position(chunk_id).unwrap(),
                geometry: chunk::Geometry::new(),
                block_list: Self::setup_block_list(grid, chunk_id),
                visibility_list: (0..grid.chunk_volume).map(|_| Vec::new()).collect(),
            })
            .collect()
    }

    fn setup_block_list(grid: &grid::Grid, chunk_id: chunk::ID) -> Vec<block::Block> {
        grid.block_ids()
            .into_iter()
            .map(|block_id| block::Block {
                id: block_id,
                position: grid.ids_to_position(chunk_id, block_id).unwrap(),
                kind: block::Kind::Empty,
                solid: false,
            })
            .collect()
    }

    pub fn get_chunk(&self, chunk_id: chunk::ID) -> Option<&chunk::Chunk> {
        self.chunk_list.get(usize::from(chunk_id))
    }

    pub fn get_chunk_mut(&mut self, chunk_id: chunk::ID) -> Option<&mut chunk::Chunk> {
        self.chunk_list.get_mut(usize::from(chunk_id))
    }

    pub fn get_chunk_at(&self, position: IVec3) -> Option<&chunk::Chunk> {
        let chunk_id = self.grid.position_to_chunk_id(position)?;

        self.get_chunk(chunk_id)
    }

    pub fn get_block(&self, chunk_id: chunk::ID, block_id: block::ID) -> Option<&block::Block> {
        let chunk = self.get_chunk(chunk_id)?;

        chunk.get_block(block_id)
    }

    pub fn get_block_at(&self, position: IVec3) -> Option<&block::Block> {
        let (chunk_id, block_id) = self.grid.position_to_ids(position)?;

        self.get_block(chunk_id, block_id)
    }

    pub fn get_clearance(&self, position: IVec3) -> Option<u32> {
        let ground_is_solid = self
            .get_block_at(position + IVec3::NEG_Y)
            .map_or(false, |block| block.solid);

        if ground_is_solid {
            let mut clearance = 0;

            for level in 0..MAXIMUM_CLEARANCE {
                let vertical_position = position + IVec3::Y * level as i32;

                let is_not_solid = self
                    .get_block_at(vertical_position)
                    .map(|block| !block.solid)
                    .unwrap_or(false);

                if is_not_solid {
                    clearance += 1;
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

            for direction in grid::Direction::face_list() {
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
        let block_meta = self.block_meta_map.get(&kind).cloned().unwrap();

        if let Some((chunk_id, block_id)) = self.grid.position_to_ids(position) {
            let chunk = self.get_chunk_mut(chunk_id).unwrap();
            let block = chunk.get_block_mut(block_id).unwrap();

            block.kind = kind;
            block.solid = block_meta.solid;

            self.update_visibility_lists(chunk_id, block_id, position);
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
                        self.set_block_kind(IVec3::new(x, y, z), kind);
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
                    self.set_block_kind(IVec3::new(x, y, z), kind);
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
        let mut chunk_geometry_update_list = Vec::new();
        let mut chunk_boundary_update_list = Vec::new();

        for chunk in &self.chunk_list {
            if chunk.modified.block {
                let chunk_geometry = self.update_chunk_geometry(chunk);

                chunk_geometry_update_list.push((chunk.id, chunk_geometry));

                let chunk_graph = self.update_chunk_graph(chunk);

                self.graph.add_chunk_graph(chunk.id, chunk_graph);
            }

            if chunk.modified.boundary {
                chunk_boundary_update_list.push(chunk.id);
            }
        }

        for (chunk_id, geometry) in chunk_geometry_update_list {
            if let Some(chunk) = self.get_chunk_mut(chunk_id) {
                chunk.geometry = geometry;

                chunk.modified.block = false;
            }
        }

        for chunk_id in chunk_boundary_update_list {
            self.update_chunk_edges(chunk_id);
            self.set_boundary_modified(chunk_id, false);
        }
    }

    fn update_chunk_graph(&self, chunk: &chunk::Chunk) -> chunk::Graph {
        let mut chunk_graph = chunk::Graph::new();

        for offset in grid::Grid::offsets_in(self.grid.chunk_radius as i32) {
            let position = chunk.position + offset;

            if self
                .get_clearance(position)
                .map_or(false, |clearance| clearance >= MINIMUM_CLEARANCE)
            {
                let block_node = block::Node {
                    block_id: self.grid.position_to_block_id(position).unwrap(),
                    position,
                    group_id: 0,
                };

                chunk_graph.add_block_node(block_node);
            }
        }

        let block_node_id_list: Vec<block::ID> = chunk_graph.get_block_node_id_list().collect();

        for block_id1 in block_node_id_list {
            let block_position1 = self.grid.ids_to_position(chunk.id, block_id1).unwrap();

            for direction in grid::Direction::traversable_list() {
                let block_position2 = block_position1 + direction.offset();

                if let Some(block_id2) = self.grid.position_to_block_id(block_position2) {
                    if chunk_graph.get_block_node(block_id2).is_some() {
                        if let Some(clearance) = self
                            .get_clearance(block_position1)
                            .zip(self.get_clearance(block_position2))
                            .map(|(clearance1, clearance2)| clearance1.min(clearance2))
                            .filter(|clearance| clearance >= &MINIMUM_CLEARANCE)
                        {
                            let cost = direction.cost();

                            chunk_graph.add_edge(
                                block_id1,
                                block_position1,
                                block_id2,
                                block_position2,
                                clearance,
                                cost,
                            );
                        }
                    }
                }
            }
        }

        let group_id_map = self.update_group_id_map(&chunk_graph);

        for (block_id, group_id) in group_id_map {
            let block_node = chunk_graph.get_block_node_mut(block_id).unwrap();

            block_node.group_id = group_id;
        }

        chunk_graph
    }

    fn update_chunk_edges(&mut self, chunk_id1: chunk::ID) {
        self.graph.clear_edges(chunk_id1);

        let chunk_radius = self.grid.chunk_radius as i32;
        let chunk_position = self.grid.chunk_id_to_position(chunk_id1).unwrap();

        for offset in grid::Grid::offsets_in(chunk_radius) {
            let block_position1 = chunk_position + offset;

            if let Some(clearance1) = self
                .get_clearance(block_position1)
                .filter(|clearance| clearance >= &MINIMUM_CLEARANCE)
            {
                for direction in grid::Direction::traversable_list() {
                    let block_position2 = block_position1 + direction.offset();

                    if let Some(chunk_id2) = self.grid.position_to_chunk_id(block_position2) {
                        if chunk_id1 == chunk_id2 {
                            continue;
                        }

                        if let Some(clearance2) = self
                            .get_clearance(block_position2)
                            .filter(|clearance| clearance >= &MINIMUM_CLEARANCE)
                        {
                            let clearance = clearance1.min(clearance2);

                            let block_id1 =
                                self.grid.position_to_block_id(block_position1).unwrap();

                            let block_id2 =
                                self.grid.position_to_block_id(block_position2).unwrap();

                            let cost = if block_position1.y == block_position2.y {
                                MOVEMENT_COST_FACE
                            } else {
                                MOVEMENT_COST_EDGE
                            };

                            self.graph.add_edge(
                                chunk_id1,
                                block_id1,
                                block_position1,
                                chunk_id2,
                                block_id2,
                                block_position2,
                                clearance,
                                cost,
                            );
                        }
                    }
                }
            }
        }
    }

    fn update_group_id_map(&self, chunk_graph: &chunk::Graph) -> HashMap<block::ID, u32> {
        let mut group_id = 0;
        let mut group_id_map = HashMap::new();

        let mut visited = HashSet::new();

        for block_id in chunk_graph.get_block_node_id_list() {
            if visited.contains(&block_id) {
                continue;
            }

            let mut queue = VecDeque::new();
            queue.push_back(block_id);

            while let Some(test_block_id) = queue.pop_front() {
                if !visited.insert(test_block_id) {
                    continue;
                }

                group_id_map.insert(test_block_id, group_id);

                for edge in chunk_graph.get_edge_iter(test_block_id) {
                    let target_block_id = if test_block_id == edge.block_id1 {
                        edge.block_id2
                    } else {
                        edge.block_id1
                    };

                    if !visited.contains(&target_block_id) {
                        queue.push_back(target_block_id);
                    }
                }
            }

            group_id += 1;
        }

        group_id_map
    }

    fn update_chunk_geometry(&self, chunk: &chunk::Chunk) -> chunk::Geometry {
        let mut chunk_geometry = chunk::Geometry::new();

        for block_id in self.grid.block_ids() {
            let block = self.get_block(chunk.id, block_id).unwrap();

            if block.solid {
                let visibility_list = &chunk.visibility_list[usize::from(block_id)];
                let position = self.grid.ids_to_position(chunk.id, block_id).unwrap();

                for direction in grid::Direction::face_list() {
                    if visibility_list.contains(&direction) {
                        let mut face = block::Face::new(position, direction, block.kind);

                        let (edges, corners) =
                            Self::get_face_neighbors(direction, &visibility_list);

                        face.light = Self::calculate_face_light(edges, corners);

                        chunk_geometry.face_list.push(face);
                    }
                }
            }
        }

        chunk_geometry
    }

    fn update_visibility_lists(
        &mut self,
        chunk_id: chunk::ID,
        block_id: block::ID,
        position: IVec3,
    ) {
        let mut visibility_updates_map: HashMap<chunk::ID, Vec<(block::ID, Vec<grid::Direction>)>> =
            HashMap::new();

        if self
            .get_block(chunk_id, block_id)
            .map_or(false, |block| block.kind != block::Kind::Empty)
        {
            let visibility_list = self.compute_visibility_list(position);

            visibility_updates_map
                .entry(chunk_id)
                .or_insert_with(Vec::new)
                .push((block_id, visibility_list));
        }

        for offset in grid::Direction::face_offsets() {
            let neighbor_position = position + offset;

            if let Some((chunk_id, block_id)) = self.grid.position_to_ids(neighbor_position) {
                if self
                    .get_block(chunk_id, block_id)
                    .map_or(false, |block| block.kind != block::Kind::Empty)
                {
                    let visibility_list = self.compute_visibility_list(neighbor_position);

                    visibility_updates_map
                        .entry(chunk_id)
                        .or_insert_with(Vec::new)
                        .push((block_id, visibility_list));
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

    fn compute_visibility_list(&self, position: IVec3) -> Vec<grid::Direction> {
        grid::Direction::face_list()
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

    pub fn get_visible_chunk_id_list(&self, judge: &Judge) -> Vec<chunk::ID> {
        let mut visible_chunk_id_list = Vec::new();

        if let Some(chunk_coordinates) = self.grid.chunk_id_to_chunk_coordinates(judge.chunk_id) {
            let chunk_view_radius =
                (JUDGE_VIEW_RADIUS / self.grid.chunk_size as f32).floor() as i32;
            let chunk_view_radius_squared = chunk_view_radius * chunk_view_radius;

            for x in -chunk_view_radius..=chunk_view_radius {
                for y in -chunk_view_radius..=chunk_view_radius {
                    for z in -chunk_view_radius..=chunk_view_radius {
                        let offset = IVec3::new(x, y, z);
                        let test_chunk_coordinates = chunk_coordinates + offset;

                        let distance_squared = offset.length_squared();

                        if distance_squared <= chunk_view_radius_squared {
                            if let Some(test_chunk_id) = self
                                .grid
                                .chunk_coordinates_to_chunk_id(test_chunk_coordinates)
                            {
                                visible_chunk_id_list.push(test_chunk_id);
                            }
                        }
                    }
                }
            }
        }

        visible_chunk_id_list
    }
}
