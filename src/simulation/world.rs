//! The Simulation module contains all of the logic required to generate and evolve
//! the core civilizational garden.

pub mod block;
pub mod builder;
pub mod chunk;
pub mod edge;
pub mod graph;
pub mod grid;
pub mod node;

pub use edge::Edge;
pub use graph::Graph;
pub use node::Node;

use crate::simulation::{
    consts::*,
    population::agent::{self},
    time::Tick,
    world, BLOCK_MAP,
};
use glam::{IVec3, Vec4};
use std::collections::{HashMap, HashSet};

pub struct World {
    pub tick: Tick,
    pub grid: grid::Grid,
    pub graph: world::Graph,
    pub chunk_list: Vec<chunk::Chunk>,
    pub flags: HashMap<agent::Kind, IVec3>,
}

impl World {
    pub fn new(radius: u32, chunk_radius: u32) -> World {
        let tick = Tick::ZERO;
        let grid = grid::Grid::new(radius, chunk_radius);
        let graph = world::Graph::new();
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
            graph,
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
            builder::TestWorld::build(self)
        } else {
            builder::MainWorld::build(self);
        }
    }

    pub fn tick(&mut self, tick: &Tick) {
        self.tick = *tick;
    }

    fn set_graph_update(&mut self, chunk_id: chunk::ID) {
        if let Some(chunk) = self.get_chunk_mut(chunk_id) {
            chunk.graph_updated = true;
        }
    }

    fn set_boundary_update(&mut self, chunk_id: chunk::ID) {
        if let Some(chunk) = self.get_chunk_mut(chunk_id) {
            chunk.boundary_updated = true;
        }
    }

    pub fn update_chunks(&mut self) {
        let mut chunk_graph_updates = Vec::new();

        for chunk in &self.chunk_list {
            if chunk.graph_updated {
                let graph = self.update_chunk_graph(chunk);
                let geometry = self.update_chunk_geometry(chunk);

                chunk_graph_updates.push((chunk.id, graph, geometry));
            }
        }

        for (chunk_id, graph, geometry) in chunk_graph_updates {
            if let Some(chunk) = self.get_chunk_mut(chunk_id) {
                chunk.graph = graph;
                chunk.geometry = geometry;

                chunk.graph_updated = false;
            }
        }

        let mut world_graph = self.graph.clone();

        for chunk in &self.chunk_list {
            if chunk.boundary_updated {
                self.update_world_node(chunk, &mut world_graph);
            }
        }

        for chunk in self.chunk_list.iter_mut() {
            chunk.boundary_updated = false;
        }
    }

    fn update_world_node(&self, chunk: &chunk::Chunk, world_graph: &mut world::Graph) {
        let chunk_radius = self.grid.chunk_radius as i32;

        if let Some(chunk_grid_position) = self.grid.chunk_to_grid(chunk.position) {
            let mut visited = HashSet::new();

            for cx in [-chunk_radius, chunk_radius] {
                for cy in -chunk_radius..=chunk_radius {
                    for cz in -chunk_radius..=chunk_radius {
                        let grid_position = chunk_grid_position + IVec3::new(cx, cy, cz);

                        self.generate_world_edges(grid_position, chunk, world_graph, &mut visited);
                    }
                }
            }

            for cx in -chunk_radius..=chunk_radius {
                for cy in [-chunk_radius, chunk_radius] {
                    for cz in -chunk_radius..=chunk_radius {
                        let grid_position = chunk_grid_position + IVec3::new(cx, cy, cz);

                        self.generate_world_edges(grid_position, chunk, world_graph, &mut visited);
                    }
                }
            }

            for cx in chunk_radius..=chunk_radius {
                for cy in chunk_radius..=chunk_radius {
                    for cz in [chunk_radius, chunk_radius] {
                        let grid_position = chunk_grid_position + IVec3::new(cx, cy, cz);

                        self.generate_world_edges(grid_position, chunk, world_graph, &mut visited);
                    }
                }
            }
        }
    }

    fn generate_world_edges(
        &self,
        grid_position: IVec3,
        chunk: &chunk::Chunk,
        world_graph: &mut world::Graph,
        visited: &mut HashSet<IVec3>,
    ) {
        if visited.insert(grid_position) {
            for dx in [-1, 1] {
                for dy in -1..=1 {
                    for dz in [-1, 1] {
                        let neighbor_grid_position = grid_position + IVec3::new(dx, dy, dz);

                        if let Some(neighbor_chunk_id) =
                            self.grid.grid_to_chunk_id(neighbor_grid_position)
                        {
                            if neighbor_chunk_id != chunk.id {
                                if let Some(neighbor_chunk) = self.get_chunk(neighbor_chunk_id) {
                                    let clearance = self.get_clearance(grid_position);
                                    let neighbor_clearance =
                                        self.get_clearance(neighbor_grid_position);

                                    let cost = if dy == 0 {
                                        WORLD_FACE_COST
                                    } else {
                                        WORLD_EDGE_COST
                                    };

                                    world_graph.create_edges(
                                        chunk.position,
                                        neighbor_chunk.position,
                                        grid_position,
                                        neighbor_grid_position,
                                        clearance.min(neighbor_clearance),
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

    fn setup_chunks(grid: &grid::Grid) -> Vec<chunk::Chunk> {
        let chunk_list = (0..grid.volume)
            .map(|index| {
                let chunk_id = chunk::ID(index);
                let chunk_position = grid.chunk_id_to_position(chunk_id).unwrap();

                chunk::Chunk {
                    id: chunk_id,
                    tick: Tick::ZERO,
                    graph_updated: false,
                    boundary_updated: false,
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

            self.set_graph_update(chunk_id);

            if self.grid.on_chunk_boundary(grid_position) {
                self.set_boundary_update(chunk_id);
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
                        self.set_block_kind(x, y, z, kind);
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
        let visibility_list: Vec<grid::Direction> = grid::Direction::face_list()
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
        let mut chunk_graph = chunk::Graph::new();

        for block_id in self.block_ids() {
            if let Some(block) = self.get_block(chunk.id, block_id) {
                if block.solid {
                    if let Some(grid_position) = self.grid.ids_to_grid(chunk.id, block_id) {
                        let clearance = self.get_clearance(grid_position);
                        let edge_list = Vec::new();

                        if clearance >= MINIMUM_CLEARANCE {
                            let node = chunk::Node {
                                grid_position,
                                clearance,
                                edge_list,
                            };

                            chunk_graph.add_node(grid_position, node);
                        }
                    }
                }
            }
        }

        let node_set: HashMap<IVec3, chunk::Node> = chunk_graph.node_map.clone();

        for (grid_position, node) in &node_set {
            for direction in grid::Direction::traversable_list() {
                let neighbor_grid_position = grid_position + direction.offset();

                if let Some(neighbor_node) = node_set.get(&neighbor_grid_position) {
                    chunk_graph.add_edge(
                        *grid_position,
                        chunk::Edge {
                            target: neighbor_grid_position,
                            clearance: node.clearance.min(neighbor_node.clearance),
                            cost: direction.cost(),
                        },
                    );
                }
            }
        }

        chunk_graph
    }

    fn update_chunk_geometry(&self, chunk: &chunk::Chunk) -> chunk::Geometry {
        let mut face_list = Vec::new();

        for block_id in self.block_ids() {
            let block = self.get_block(chunk.id, block_id).unwrap();

            if block.solid {
                let visibility_list = &chunk.visibility_list[usize::from(block_id)];
                let grid_position = self.grid.ids_to_grid(chunk.id, block_id).unwrap();

                for direction in grid::Direction::face_list() {
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

    pub fn get_clearance(&self, grid_position: IVec3) -> i32 {
        let base_is_solid = self
            .get_block_at(grid_position)
            .map(|block| block.solid)
            .unwrap_or(false);

        if base_is_solid {
            let mut clearance = 0;

            for level in 1..=MAXIMUM_CLEARANCE {
                let vertical_grid_position = grid_position + IVec3::Y * level as i32;

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
