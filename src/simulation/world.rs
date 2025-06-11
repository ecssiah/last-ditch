//! The Simulation module contains all of the logic required to generate and evolve
//! the core civilizational garden.

pub mod block;
pub mod builder;
pub mod chunk;
pub mod graph;
pub mod grid;

pub use graph::Graph;
pub use grid::Grid;

use crate::simulation::{
    consts::*,
    population::agent::{self},
    time::Tick,
    world, BLOCK_MAP,
};
use glam::{IVec3, Vec4};
use std::collections::{HashMap, HashSet, VecDeque};

pub struct World {
    pub(crate) tick: Tick,
    pub(crate) grid: grid::Grid,
    pub(crate) chunk_list: Vec<chunk::Chunk>,
    pub(crate) graph: world::Graph,
    pub(crate) flags: HashMap<agent::Kind, IVec3>,
}

impl World {
    pub fn new(chunk_radius: u32, world_radius: u32) -> World {
        let tick = Tick::ZERO;
        let grid = grid::Grid::new(chunk_radius, world_radius);
        let chunk_list = Self::setup_chunks(&grid);
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

    fn setup_chunks(grid: &grid::Grid) -> Vec<chunk::Chunk> {
        grid.chunk_ids()
            .into_iter()
            .map(|chunk_id| chunk::Chunk {
                id: chunk_id,
                tick: Tick::ZERO,
                block_updated: false,
                boundary_updated: false,
                position: grid.chunk_id_to_position(chunk_id).unwrap(),
                geometry: chunk::Geometry::new(),
                kind_list: Vec::from([block::Kind::Empty]),
                block_list: (0..grid.chunk_volume).map(|_| 0).collect(),
                visibility_list: (0..grid.chunk_volume).map(|_| Vec::new()).collect(),
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

        let kind_id = *chunk.block_list.get(usize::from(block_id))?;
        let kind = chunk.kind_list.get(usize::from(kind_id))?;

        BLOCK_MAP.get(&kind)
    }

    pub fn get_block_at(&self, position: IVec3) -> Option<&block::Block> {
        let (chunk_id, block_id) = self.grid.position_to_ids(position)?;

        self.get_block(chunk_id, block_id)
    }

    fn get_kind_id(chunk: &chunk::Chunk, kind: block::Kind) -> Option<usize> {
        chunk
            .kind_list
            .iter()
            .position(|target_kind| kind == *target_kind)
    }

    pub fn has_clearance(&self, position: IVec3, height: u32) -> bool {
        let test_height = height.min(MAXIMUM_CLEARANCE);

        (0..test_height).all(|level| {
            let vertical_position = position + level as i32 * IVec3::Y;

            self.get_block_at(vertical_position)
                .map(|block| !block.solid)
                .unwrap_or(false)
        })
    }

    pub fn get_clearance(&self, position: IVec3) -> u32 {
        let mut clearance = 0;

        for level in 0..MAXIMUM_CLEARANCE {
            let vertical_position = position + IVec3::Y * level as i32;

            let is_clear = self
                .get_block_at(vertical_position)
                .map(|block| !block.solid)
                .unwrap_or(false);

            if is_clear {
                clearance += 1;
            } else {
                break;
            }
        }

        clearance
    }

    pub fn set_block_kind(&mut self, position: IVec3, kind: block::Kind) {
        if let Some((chunk_id, block_id)) = self.grid.position_to_ids(position) {
            self.update_kind_list(chunk_id, block_id, kind);
            self.update_visibility_lists(chunk_id, block_id, position);

            self.mark_updates(chunk_id, position);
        }
    }

    fn mark_updates(&mut self, chunk_id: chunk::ID, position: IVec3) {
        self.set_block_updated(chunk_id, true);

        let directions = self.grid.boundary_contact_direction_list(position);

        if directions.len() > 0 {
            self.set_boundary_updated(chunk_id, true);

            if let Some(chunk_coordinates) = self.grid.chunk_id_to_chunk_coordinates(chunk_id) {
                for direction in directions {
                    let boundary_chunk_coordinates = chunk_coordinates + direction.offset();

                    if let Some(boundary_chunk_id) = self
                        .grid
                        .chunk_coordinates_to_chunk_id(boundary_chunk_coordinates)
                    {
                        self.set_boundary_updated(boundary_chunk_id, true);
                    }
                }
            }
        }
    }

    pub fn set_box(&mut self, point1: IVec3, point2: IVec3, kind: block::Kind) {
        let min = point1.min(point2);
        let max = point1.max(point2);

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

    pub fn set_cube(&mut self, point1: IVec3, point2: IVec3, kind: block::Kind) {
        let min = point1.min(point2);
        let max = point1.max(point2);

        for x in min.x..=max.x {
            for y in min.y..=max.y {
                for z in min.z..=max.z {
                    self.set_block_kind(IVec3::new(x, y, z), kind);
                }
            }
        }
    }

    fn set_block_updated(&mut self, chunk_id: chunk::ID, updated: bool) {
        if let Some(chunk) = self.get_chunk_mut(chunk_id) {
            chunk.block_updated = updated;
        }
    }

    fn set_boundary_updated(&mut self, chunk_id: chunk::ID, updated: bool) {
        if let Some(chunk) = self.get_chunk_mut(chunk_id) {
            chunk.boundary_updated = updated;
        }
    }

    pub fn update_chunks(&mut self) {
        let mut chunk_geometry_update_list = Vec::new();
        let mut chunk_boundary_update_list = Vec::new();

        for chunk in &self.chunk_list {
            if chunk.block_updated {
                let chunk_graph = self.update_chunk_graph(chunk);

                self.graph.add_chunk_graph(chunk.id, chunk_graph);

                let chunk_geometry = self.update_chunk_geometry(chunk);

                chunk_geometry_update_list.push((chunk.id, chunk_geometry));
            }

            if chunk.boundary_updated {
                chunk_boundary_update_list.push(chunk.id);
            }
        }

        for (chunk_id, geometry) in chunk_geometry_update_list {
            if let Some(chunk) = self.get_chunk_mut(chunk_id) {
                chunk.geometry = geometry;

                chunk.block_updated = false;
            }
        }

        for chunk_id in chunk_boundary_update_list {
            self.update_chunk_edges(chunk_id);
            self.set_boundary_updated(chunk_id, false);
        }
    }

    fn update_chunk_graph(&self, chunk: &chunk::Chunk) -> chunk::Graph {
        let mut chunk_graph = chunk::Graph::new();

        let chunk_radius = self.grid.chunk_radius as i32;

        for offset in grid::Grid::offsets_in(chunk_radius) {
            let position = chunk.position + offset;

            let ground_is_solid = self
                .get_block_at(position + IVec3::NEG_Y)
                .map_or(false, |block| block.solid);

            if ground_is_solid {
                let clearance = self.get_clearance(position);

                if clearance >= MINIMUM_CLEARANCE {
                    let block_node = block::Node {
                        block_id: self.grid.position_to_block_id(position).unwrap(),
                        position: position,
                        group_id: 0,
                    };

                    chunk_graph.add_block_node(block_node.block_id, block_node);
                }
            }
        }

        let mut edge_data_list = Vec::new();
        let block_node_ids: HashSet<block::ID> = chunk_graph.get_node_block_ids().collect();

        for block_id1 in &block_node_ids {
            let block_id1 = *block_id1;
            let block_position1 = self.grid.ids_to_position(chunk.id, block_id1).unwrap();

            for direction in grid::Direction::traversable_list() {
                let block_position2 = block_position1 + direction.offset();
                let block_id2 = self.grid.position_to_block_id(block_position2).unwrap();

                if !block_node_ids.contains(&block_id2) {
                    continue;
                }

                let clearance1 = self.get_clearance(block_position1);
                let clearance2 = self.get_clearance(block_position2);
                let clearance = clearance1.min(clearance2);

                if clearance >= MINIMUM_CLEARANCE {
                    let cost = direction.cost();

                    edge_data_list.push((
                        block_id1,
                        block_position1,
                        block_id2,
                        block_position2,
                        clearance,
                        cost,
                    ));
                }
            }
        }

        for (block_id1, block_position1, block_id2, block_position2, clearance, cost) in
            edge_data_list
        {
            chunk_graph.add_edge(
                block_id1,
                block_position1,
                block_id2,
                block_position2,
                clearance,
                cost,
            );
        }

        let group_id_map = self.update_group_id_map(chunk);

        for (block_id, group_id) in group_id_map {
            if let Some(block_node) = chunk_graph.get_block_node_mut(block_id) {
                block_node.group_id = group_id;
            }
        }

        chunk_graph
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

    fn update_chunk_edges(&mut self, chunk_id: chunk::ID) {
        if let Some(chunk_position) = self.grid.chunk_id_to_position(chunk_id) {
            self.graph.clear_edges(chunk_id);

            let chunk_radius = self.grid.chunk_radius as i32;

            for offset in grid::Grid::offsets_in(chunk_radius) {
                let block_position1 = chunk_position + offset;

                if let Some(block1) = self.get_block_at(block_position1 + IVec3::NEG_Y) {
                    if block1.solid {
                        let clearance1 = self.get_clearance(block_position1);

                        if clearance1 >= MINIMUM_CLEARANCE {
                            let boundary_directions =
                                self.grid.boundary_contact_direction_list(block_position1);

                            for direction in boundary_directions {
                                if [grid::Direction::XoYpZo, grid::Direction::XoYnZo]
                                    .contains(&direction)
                                {
                                    continue;
                                }

                                for dy in [-1, 0, 1] {
                                    let y_offset = direction.offset() + IVec3::Y * dy;
                                    let block_position2 = block_position1 + y_offset;

                                    if let Some(block2) =
                                        self.get_block_at(block_position2 + IVec3::NEG_Y)
                                    {
                                        if block2.solid {
                                            let clearance2 = self.get_clearance(block_position2);
                                            let clearance = clearance1.min(clearance2);

                                            if clearance >= MINIMUM_CLEARANCE {
                                                let (chunk_id1, block_id1) = self
                                                    .grid
                                                    .position_to_ids(block_position1)
                                                    .unwrap();

                                                let (chunk_id2, block_id2) = self
                                                    .grid
                                                    .position_to_ids(block_position2)
                                                    .unwrap();

                                                let cost =
                                                    if block_position1.y - block_position2.y == 0 {
                                                        WORLD_FACE_COST
                                                    } else {
                                                        WORLD_EDGE_COST
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
                    }
                }
            }
        }
    }

    // fn update_world_edges(&self, chunk: &chunk::Chunk) -> HashMap<block::ID, Vec<chunk::Edge>> {
    //     let mut world_edges_map = HashMap::new();

    //     let chunk_radius = self.grid.chunk_radius as i32;

    //     for offset in grid::Grid::offsets_in(chunk_radius) {
    //         let position = chunk.position + offset;

    //         if let Some(block_id) = self.grid.position_to_block_id(position) {
    //             if self.grid.boundary_contact_direction_list(position).len() > 0 {
    //                 let chunk_edge_list = self.update_chunk_edge_list(position, chunk);

    //                 world_edges_map.insert(block_id, chunk_edge_list);
    //             }
    //         }
    //     }

    //     world_edges_map
    // }

    // fn update_chunk_edge_list(&self, position1: IVec3, chunk: &chunk::Chunk) -> Vec<chunk::Edge> {
    //     let mut chunk_edge_list = Vec::new();

    //     for direction in grid::Direction::traversable_list() {
    //         let position2 = position1 + direction.offset();

    //         if let Some(neighbor_chunk_id) = self.grid.position_to_chunk_id(position2) {
    //             if neighbor_chunk_id == chunk.id {
    //                 continue;
    //             }

    //             let clearance1 = self.get_clearance(position1);
    //             let clearance2 = self.get_clearance(position2);

    //             let clearance = clearance1.min(clearance2);

    //             if clearance >= MINIMUM_CLEARANCE {
    //                 let ((chunk_id1, chunk_id2), (position1, position2)) =
    //                     if chunk.id < neighbor_chunk_id {
    //                         ((chunk.id, neighbor_chunk_id), (position1, position2))
    //                     } else {
    //                         ((neighbor_chunk_id, chunk.id), (position2, position1))
    //                     };

    //                 let cost = direction.cost();

    //                 let chunk_edge = chunk::Edge {
    //                     chunk_id1,
    //                     chunk_id2,
    //                     position1,
    //                     position2,
    //                     clearance,
    //                     cost,
    //                 };

    //                 chunk_edge_list.push(chunk_edge);
    //             }
    //         }
    //     }

    //     chunk_edge_list
    // }

    fn update_group_id_map(&self, chunk: &chunk::Chunk) -> HashMap<block::ID, u32> {
        let mut group_id = 0;
        let mut group_id_map: HashMap<block::ID, u32> = HashMap::new();

        let mut visited: HashSet<block::ID> = HashSet::new();

        if let Some(chunk_graph) = self.graph.get_chunk_graph(chunk.id) {
            for block_id in chunk_graph.get_node_block_ids() {
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

                    // if let Some(node) = chunk_graph.get_block_node(test_block_id) {
                    //     for edge in &node.edge_list {
                    //         let position2 = edge.position2;

                    //         if !visited.contains(&position2) {
                    //             queue.push_back(position2);
                    //         }
                    //     }
                    // }
                }

                group_id += 1;
            }
        }

        group_id_map
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

    fn update_visibility_lists(
        &mut self,
        chunk_id: chunk::ID,
        block_id: block::ID,
        position: IVec3,
    ) {
        let mut visibility_updates_map: HashMap<chunk::ID, Vec<(block::ID, Vec<grid::Direction>)>> =
            HashMap::new();

        if let Some(block) = self.get_block(chunk_id, block_id) {
            if block.kind != block::Kind::Empty {
                let visibility_list = self.compute_visibility_list(position);

                visibility_updates_map
                    .entry(chunk_id)
                    .or_insert_with(Vec::new)
                    .push((block_id, visibility_list));
            }
        }

        for offset in grid::Direction::face_offsets() {
            let neighbor_position = position + offset;

            if let Some((chunk_id, block_id)) = self.grid.position_to_ids(neighbor_position) {
                if let Some(block) = self.get_block(chunk_id, block_id) {
                    if block.kind != block::Kind::Empty {
                        let visibility_list = self.compute_visibility_list(neighbor_position);

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

    fn compute_visibility_list(&self, position: IVec3) -> Vec<grid::Direction> {
        let visibility_list: Vec<grid::Direction> = grid::Direction::face_list()
            .iter()
            .filter_map(|&direction| {
                let neighbor_position = position + direction.offset();
                let block = self.get_block_at(neighbor_position);

                block
                    .filter(|block| block.kind == block::Kind::Empty)
                    .map(|_| direction)
            })
            .collect();

        visibility_list
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

    pub fn get_visible_chunk_id_list(&self, chunk_id: chunk::ID) -> Vec<chunk::ID> {
        let radius = JUDGE_VIEW_RADIUS as i32;
        let radius_squared = radius * radius;
        let chunk_count_estimate = ((2 * radius + 1).pow(3)) as usize;

        let mut visible_chunk_id_list = Vec::with_capacity(chunk_count_estimate);

        if let Some(chunk_coordinates) = self.grid.chunk_id_to_chunk_coordinates(chunk_id) {
            for offset in grid::Grid::offsets_in(radius) {
                if offset.length_squared() < radius_squared {
                    let visible_chunk_coordinates = chunk_coordinates + offset;

                    if let Some(visible_chunk_id) = self
                        .grid
                        .chunk_coordinates_to_chunk_id(visible_chunk_coordinates)
                    {
                        visible_chunk_id_list.push(visible_chunk_id);
                    }
                }
            }
        }

        visible_chunk_id_list
    }
}
