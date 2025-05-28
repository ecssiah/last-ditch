pub mod block;
pub mod chunk;
pub mod grid;

use crate::simulation::{
    consts::*,
    population::agent::{self},
    time::Tick,
    world::chunk::Chunk,
    BLOCK_MAP,
};
use glam::{IVec3, Vec4};
use std::collections::HashMap;

pub struct World {
    pub tick: Tick,
    pub chunk_list: [Chunk; WORLD_VOLUME],
    pub flags: HashMap<agent::kind::Kind, IVec3>,
}

impl World {
    pub fn new() -> World {
        let tick = Tick::ZERO;
        let chunk_list = Self::setup_chunks();
        let flags = HashMap::from([
            (agent::kind::Kind::Lion, IVec3::ZERO),
            (agent::kind::Kind::Eagle, IVec3::ZERO),
            (agent::kind::Kind::Horse, IVec3::ZERO),
            (agent::kind::Kind::Wolf, IVec3::ZERO),
        ]);

        let world = Self {
            tick,
            chunk_list,
            flags,
        };

        world
    }

    pub fn get_flag(&self, kind: &agent::Kind) -> Option<IVec3> {
        self.flags.get(kind).cloned()
    }

    pub fn setup(&mut self) {
        log::info!("Setup Ground");

        self.setup_ground();

        log::info!("Setup Structures");

        self.set_block_kind(0, 0, 0, &block::Kind::Origin);
        self.set_block_kind(0, 0, 4, &block::Kind::North);
        self.set_block_kind(-4, 0, 0, &block::Kind::West);
        self.set_block_kind(0, 0, -4, &block::Kind::South);
        self.set_block_kind(4, 0, 0, &block::Kind::East);

        self.setup_temple(0, 2, 34, &agent::Kind::Eagle);
        self.setup_temple(-34, 2, 0, &agent::Kind::Lion);
        self.setup_temple(0, 2, -34, &agent::Kind::Horse);
        self.setup_temple(34, 2, 0, &agent::Kind::Wolf);

        self.update_chunk_meshes();
    }

    pub fn tick(&mut self, tick: &Tick) {
        self.tick = *tick;
    }

    fn setup_chunks() -> [Chunk; WORLD_VOLUME] {
        let chunk_list: [Chunk; WORLD_VOLUME] = core::array::from_fn(|index| {
            let chunk_id = chunk::ID(index);
            let chunk_position = grid::chunk_id_to_position(chunk_id).unwrap();

            Chunk {
                id: chunk_id,
                tick: Tick::ZERO,
                updated: false,
                position: chunk_position,
                geometry: chunk::Geometry::new(),
                kind_list: Vec::from([block::Kind::Empty]),
                block_list: Box::new([0; CHUNK_VOLUME]),
                direction_list: Box::new([grid::Direction::XoYoZo; CHUNK_VOLUME]),
                light_list: Box::new(core::array::from_fn(|_| block::Light::new())),
                neighbor_list: Box::new(core::array::from_fn(|_| Vec::new())),
                visibility_list: Box::new(core::array::from_fn(|_| Vec::new())),
            }
        });

        chunk_list
    }

    fn update_chunk_meshes(&mut self) {
        for chunk_id in (0..WORLD_VOLUME).map(chunk::ID) {
            self.update_chunk_geometry(chunk_id);
        }
    }

    fn setup_ground(&mut self) {
        let ground_boundary = GRID_BOUNDARY as isize - CHUNK_SIZE as isize;

        for x in -ground_boundary..=ground_boundary {
            for y in -1..=0 {
                for z in -ground_boundary..=ground_boundary {
                    let x = x as i32;
                    let y = y as i32;
                    let z = z as i32;

                    let chunk_position = grid::grid_to_chunk(IVec3::new(x, y, z)).unwrap();

                    let kind = if (chunk_position.x + chunk_position.y + chunk_position.z) % 2 == 0
                    {
                        &block::Kind::Polished1
                    } else {
                        &block::Kind::Polished2
                    };

                    self.set_block_kind(x, y, z, kind);
                }
            }
        }
    }

    fn setup_temple(&mut self, x: i32, y: i32, z: i32, kind: &agent::Kind) {
        self.flags.insert(kind.clone(), IVec3::new(x, y, z));

        self.set_cube(
            IVec3::new(x - 8, y - 1, z - 8),
            IVec3::new(x + 8, y - 1, z + 8),
            &block::Kind::Stone1,
        );

        self.set_cube(
            IVec3::new(x - 7, y, z - 7),
            IVec3::new(x + 7, y, z + 7),
            &block::Kind::Stone1,
        );

        self.set_cube(
            IVec3::new(x - 6, y + 6, z - 6),
            IVec3::new(x + 6, y + 6, z + 6),
            &block::Kind::Stone1,
        );

        self.set_cube(
            IVec3::new(x - 5, y + 7, z - 5),
            IVec3::new(x + 5, y + 7, z + 5),
            &block::Kind::Stone1,
        );

        self.set_cube(
            IVec3::new(x - 5, y + 6, z - 5),
            IVec3::new(x + 5, y + 6, z + 5),
            &block::Kind::Empty,
        );

        self.set_cube(
            IVec3::new(x + 5, y - 1, z + 5),
            IVec3::new(x + 5, y + 6, z + 5),
            &block::Kind::Engraved1,
        );

        self.set_cube(
            IVec3::new(x - 5, y - 1, z + 5),
            IVec3::new(x - 5, y + 6, z + 5),
            &block::Kind::Engraved1,
        );

        self.set_cube(
            IVec3::new(x + 5, y - 1, z - 5),
            IVec3::new(x + 5, y + 6, z - 5),
            &block::Kind::Engraved1,
        );

        self.set_cube(
            IVec3::new(x - 5, y - 1, z - 5),
            IVec3::new(x - 5, y + 6, z - 5),
            &block::Kind::Engraved1,
        );

        self.set_block_kind(x, y + 4, z, kind.icon());

        self.set_cube(
            IVec3::new(x, y + 5, z),
            IVec3::new(x, y + 6, z),
            &block::Kind::Polished1,
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
        let chunk_id = grid::grid_to_chunk_id(grid_position)?;

        let chunk = self.get_chunk(chunk_id);

        chunk
    }

    pub fn get_block(&self, grid_position: IVec3) -> Option<&block::Block> {
        let (chunk_id, block_id) = grid::grid_to_ids(grid_position)?;

        let chunk = self.get_chunk(chunk_id)?;

        let kind_id = *chunk.block_list.get(usize::from(block_id))?;
        let kind = chunk.kind_list[usize::from(kind_id)];

        let block = BLOCK_MAP.get(&kind)?;

        Some(block)
    }

    pub fn set_block_kind(&mut self, x: i32, y: i32, z: i32, kind: &block::Kind) -> bool {
        let grid_position = IVec3::new(x, y, z);

        if let Some((chunk_id, block_id)) = grid::grid_to_ids(grid_position) {
            self.update_kind_list(chunk_id, block_id, kind);
            self.update_neighbor_lists(grid_position);
            self.update_visibility_lists(chunk_id, block_id, grid_position);
            self.update_light(chunk_id, block_id, grid_position);

            self.mark_update(chunk_id);

            true
        } else {
            false
        }
    }

    pub fn set_cube(&mut self, min: IVec3, max: IVec3, kind: &block::Kind) {
        for x in min.x..=max.x {
            for y in min.y..=max.y {
                for z in min.z..=max.z {
                    self.set_block_kind(x, y, z, kind);
                }
            }
        }
    }

    fn update_kind_list(&mut self, chunk_id: chunk::ID, block_id: block::ID, kind: &block::Kind) {
        if let Some(chunk) = self.get_chunk_mut(chunk_id) {
            if let Some(kind_id) = Self::get_kind_id(chunk, kind) {
                chunk.block_list[usize::from(block_id)] = kind_id;
            } else {
                chunk.kind_list.push(kind.clone());
                chunk.block_list[usize::from(block_id)] = chunk.kind_list.len() - 1;
            }
        }
    }

    fn get_kind_id(chunk: &Chunk, kind: &block::Kind) -> Option<usize> {
        let kind_id = chunk
            .kind_list
            .iter()
            .position(|target_kind| kind == target_kind)?;

        Some(kind_id)
    }

    fn update_neighbor_lists(&mut self, grid_position: IVec3) {
        let mut update_map: HashMap<chunk::ID, Vec<(block::ID, Vec<grid::Direction>)>> =
            HashMap::new();

        for direction in grid::Direction::all() {
            let neighbor_grid_position = grid_position + direction.offset();

            if let Some((chunk_id, block_id)) = grid::grid_to_ids(neighbor_grid_position) {
                let neighbor_list = self.compute_neighbor_list(neighbor_grid_position);

                update_map
                    .entry(chunk_id)
                    .or_insert_with(Vec::new)
                    .push((block_id, neighbor_list));
            }
        }

        for (chunk_id, update_list) in update_map {
            if let Some(chunk) = self.get_chunk_mut(chunk_id) {
                for (block_id, neighbor_list) in update_list {
                    chunk.neighbor_list[usize::from(block_id)] = neighbor_list;
                }
            }
        }
    }

    fn compute_neighbor_list(&mut self, grid_position: IVec3) -> Vec<grid::Direction> {
        let mut neighbor_list = Vec::new();

        for direction in grid::Direction::all() {
            if direction == grid::Direction::XoYoZo {
                continue;
            }

            let neighbor_grid_position = grid_position + direction.offset();

            if let Some(block) = self.get_block(neighbor_grid_position) {
                if block.solid {
                    neighbor_list.push(direction);
                }
            }
        }

        neighbor_list
    }

    fn update_visibility_lists(
        &mut self,
        chunk_id: chunk::ID,
        block_id: block::ID,
        grid_position: IVec3,
    ) {
        let mut visibility_updates_map: HashMap<chunk::ID, Vec<(block::ID, Vec<grid::Direction>)>> =
            HashMap::new();

        if let Some(block) = self.get_block(grid_position) {
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

            if let Some((chunk_id, block_id)) = grid::grid_to_ids(neighbor_grid_position) {
                if let Some(block) = self.get_block(neighbor_grid_position) {
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
                let block = self.get_block(neighbor_grid_position);

                block
                    .filter(|block| block.kind == block::Kind::Empty)
                    .map(|_| direction)
            })
            .collect();

        visibility_list
    }

    fn update_light(&mut self, _chunk_id: chunk::ID, _block_id: block::ID, _grid_position: IVec3) {}

    fn mark_update(&mut self, chunk_id: chunk::ID) {
        if let Some(chunk) = self.get_chunk_mut(chunk_id) {
            chunk.updated = true;
        }
    }

    fn update_chunk_geometry(&mut self, chunk_id: chunk::ID) {
        if let Some(chunk) = self.get_chunk(chunk_id) {
            if chunk.updated {
                let geometry = self.setup_chunk_geometry(chunk_id);

                if let Some(chunk) = self.get_chunk_mut(chunk_id) {
                    chunk.updated = false;
                    chunk.geometry = geometry;
                }
            }
        }
    }

    fn setup_chunk_geometry(&self, chunk_id: chunk::ID) -> chunk::Geometry {
        let mut face_list = Vec::new();
        let chunk = self.get_chunk(chunk_id).unwrap();

        for block_id in (0..CHUNK_VOLUME).map(block::ID) {
            let block = chunk.get_block(block_id).unwrap();

            if block.solid {
                let visibility_list = &chunk.visibility_list[usize::from(block_id)];
                let grid_position = grid::ids_to_grid(chunk_id, block_id).unwrap();

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
            .get_block(grid_position)
            .map(|block| block.solid)
            .unwrap_or(false);

        let clear_above = (1..=height).all(|level| {
            self.get_block(grid_position + level * IVec3::Y)
                .map(|block| !block.solid)
                .unwrap_or(false)
        });

        base_is_solid && clear_above
    }

    pub fn get_visible_chunk_id_list(chunk_id: chunk::ID) -> Vec<chunk::ID> {
        let radius = JUDGE_VIEW_RADIUS as i32;
        let chunk_count_estimate = ((2 * radius + 1).pow(3)) as usize;

        let mut visible_chunk_id_list = Vec::with_capacity(chunk_count_estimate);

        if let Some(chunk_position) = grid::chunk_id_to_position(chunk_id) {
            for dx in -radius..=radius {
                for dy in -radius..=radius {
                    for dz in -radius..=radius {
                        let offset = dx.abs() + dy.abs() + dz.abs();

                        if offset > radius {
                            continue;
                        }

                        let visible_chunk_position = chunk_position + IVec3::new(dx, dy, dz);

                        if let Some(grid_position) = grid::chunk_to_grid(visible_chunk_position) {
                            if let Some(visible_chunk_id) = grid::grid_to_chunk_id(grid_position) {
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
