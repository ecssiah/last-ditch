use crate::simulation::{
    block::{self, Direction, Face},
    chunk,
    consts::*,
    structure,
    time::Tick,
    Block, Chunk, BLOCKS,
};
use glam::{IVec3, Vec3, Vec4};
use std::collections::HashMap;

pub struct World {
    pub tick: Tick,
    pub chunks: [Chunk; WORLD_VOLUME],
}

impl World {
    pub fn new() -> World {
        let tick = Tick::ZERO;
        let chunks = Self::setup_chunks();

        let world = Self { tick, chunks };

        world
    }

    fn setup_chunks() -> [Chunk; WORLD_VOLUME] {
        let chunks: [Chunk; WORLD_VOLUME] = core::array::from_fn(|index| {
            let chunk_id = chunk::ID(index);
            let chunk_position = Chunk::position(chunk_id).unwrap();

            Chunk {
                id: chunk_id,
                position: chunk_position,
                tick: Tick::ZERO,
                updated: false,
                palette: Vec::from([block::Kind::Air]),
                blocks: Box::new([0; CHUNK_VOLUME]),
                meta: Box::new(core::array::from_fn(|_| block::Meta::new())),
                light: Box::new(core::array::from_fn(|_| block::Light::new())),
                mesh: chunk::Mesh::new(),
            }
        });

        chunks
    }

    pub fn generate(&mut self) {
        self.generate_ground();

        self.set_cube(
            IVec3::new(-6, 5, -6),
            IVec3::new(6, 5, 6),
            &block::Kind::Stone1,
        );

        self.set_cube(
            IVec3::new(-3, 5, -3),
            IVec3::new(3, 5, 3),
            &block::Kind::Air,
        );

        self.set_cube(
            IVec3::new(-5, 6, -5),
            IVec3::new(5, 6, 5),
            &block::Kind::Stone1,
        );

        self.set_block_kind(5, 5, 5, &block::Kind::Engraved2);
        self.set_block_kind(5, 4, 5, &block::Kind::Engraved1);
        self.set_block_kind(5, 3, 5, &block::Kind::Engraved2);
        self.set_block_kind(5, 2, 5, &block::Kind::Engraved1);
        self.set_block_kind(5, 1, 5, &block::Kind::Engraved2);

        self.set_block_kind(-5, 5, 5, &block::Kind::Engraved2);
        self.set_block_kind(-5, 4, 5, &block::Kind::Engraved1);
        self.set_block_kind(-5, 3, 5, &block::Kind::Engraved2);
        self.set_block_kind(-5, 2, 5, &block::Kind::Engraved1);
        self.set_block_kind(-5, 1, 5, &block::Kind::Engraved2);

        self.set_block_kind(5, 5, -5, &block::Kind::Engraved2);
        self.set_block_kind(5, 4, -5, &block::Kind::Engraved1);
        self.set_block_kind(5, 3, -5, &block::Kind::Engraved2);
        self.set_block_kind(5, 2, -5, &block::Kind::Engraved1);
        self.set_block_kind(5, 1, -5, &block::Kind::Engraved2);

        self.set_block_kind(-5, 5, -5, &block::Kind::Engraved2);
        self.set_block_kind(-5, 4, -5, &block::Kind::Engraved1);
        self.set_block_kind(-5, 3, -5, &block::Kind::Engraved2);
        self.set_block_kind(-5, 2, -5, &block::Kind::Engraved1);
        self.set_block_kind(-5, 1, -5, &block::Kind::Engraved2);

        self.update_chunk_meshes();
    }

    fn update_chunk_meshes(&mut self) {
        for chunk_id in (0..WORLD_VOLUME).map(chunk::ID) {
            self.update_chunk_mesh(chunk_id);
        }
    }

    pub fn tick(&mut self, tick: &Tick) {
        self.tick = *tick;
    }

    pub fn generate_ground(&mut self) {
        let world_boundary = WORLD_BOUNDARY as isize;

        for x in -world_boundary..=world_boundary {
            for z in -world_boundary..=world_boundary {
                let chunk_position = Chunk::position_at(IVec3::new(x as i32, 0, z as i32)).unwrap();

                let kind = if (chunk_position.x + chunk_position.z) % 2 == 0 {
                    &block::Kind::Stone1
                } else {
                    &block::Kind::Stone2
                };

                self.set_block_kind(x as i32, 0, z as i32, kind);
            }
        }
    }

    pub fn generate_structure(&mut self, x: i32, y: i32, z: i32, structure_kind: &structure::Kind) {
        if let Some(structure) = STRUCTURES.get(structure_kind) {
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
                    &block_data.kind,
                );
            }
        }
    }

    pub fn get_chunk(&self, chunk_id: chunk::ID) -> Option<&chunk::Chunk> {
        let chunk = self.chunks.get(usize::from(chunk_id))?;

        Some(chunk)
    }

    pub fn get_chunk_mut(&mut self, chunk_id: chunk::ID) -> Option<&mut chunk::Chunk> {
        let chunk = self.chunks.get_mut(usize::from(chunk_id))?;

        Some(chunk)
    }

    pub fn get_chunk_at(&self, grid_position: IVec3) -> Option<&chunk::Chunk> {
        let chunk_id = Chunk::id_at_grid(grid_position)?;

        let chunk = self.get_chunk(chunk_id);

        chunk
    }

    pub fn get_block(&self, grid_position: IVec3) -> Option<&block::Block> {
        let (chunk_id, block_id) = World::ids_at(grid_position)?;

        let chunk = self.get_chunk(chunk_id)?;

        let palette_id = *chunk.blocks.get(usize::from(block_id))?;
        let kind = chunk.palette[usize::from(palette_id)];

        let block = BLOCKS.get(&kind)?;

        Some(block)
    }

    pub fn set_block_kind(&mut self, x: i32, y: i32, z: i32, kind: &block::Kind) -> bool {
        let grid_position = IVec3::new(x, y, z);

        if let Some((chunk_id, block_id)) = World::ids_at(grid_position) {
            self.update_palette(chunk_id, block_id, kind);
            self.update_neighbors(grid_position);
            self.update_visibility(chunk_id, block_id, grid_position);
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

    fn update_palette(&mut self, chunk_id: chunk::ID, block_id: block::ID, kind: &block::Kind) {
        if let Some(chunk) = self.get_chunk_mut(chunk_id) {
            if let Some(palette_id) = Self::get_palette_id(chunk, kind) {
                chunk.blocks[usize::from(block_id)] = palette_id;
            } else {
                chunk.palette.push(kind.clone());
                chunk.blocks[usize::from(block_id)] = chunk.palette.len() - 1;
            }
        }
    }

    fn get_palette_id(chunk: &Chunk, kind: &block::Kind) -> Option<usize> {
        let palette_id = chunk
            .palette
            .iter()
            .position(|palette_kind| kind == palette_kind)?;

        Some(palette_id)
    }

    fn update_neighbors(&mut self, grid_position: IVec3) {
        let mut updates: HashMap<chunk::ID, Vec<(block::ID, Vec<block::Direction>)>> =
            HashMap::new();

        for direction in Direction::all() {
            let neighbor_grid_position = grid_position + direction.offset();

            if let Some((chunk_id, block_id)) = World::ids_at(neighbor_grid_position) {
                let neighbors = self.compute_neighbors(neighbor_grid_position);

                updates
                    .entry(chunk_id)
                    .or_insert_with(Vec::new)
                    .push((block_id, neighbors));
            }
        }

        for (chunk_id, updates) in updates {
            if let Some(chunk) = self.get_chunk_mut(chunk_id) {
                for (block_id, neighbors) in updates {
                    chunk.meta[usize::from(block_id)].neighbors = neighbors;
                }
            }
        }
    }

    fn compute_neighbors(&mut self, grid_position: IVec3) -> Vec<block::Direction> {
        let mut neighbors = Vec::new();

        for direction in Direction::all() {
            if direction == Direction::XoYoZo {
                continue;
            }

            let neighbor_grid_position = grid_position + direction.offset();

            if let Some(block) = self.get_block(neighbor_grid_position) {
                if block.solid {
                    neighbors.push(direction);
                }
            }
        }

        neighbors
    }

    fn update_visibility(
        &mut self,
        chunk_id: chunk::ID,
        block_id: block::ID,
        grid_position: IVec3,
    ) {
        let mut updates: HashMap<chunk::ID, Vec<(block::ID, Vec<block::Direction>)>> =
            HashMap::new();

        if let Some(block) = self.get_block(grid_position) {
            if block.kind != block::Kind::Air {
                let visibility = self.compute_visibility(grid_position);

                updates
                    .entry(chunk_id)
                    .or_insert_with(Vec::new)
                    .push((block_id, visibility));
            }
        }

        for offset in Direction::face_offsets() {
            let neighbor_grid_position = grid_position + offset;

            if let Some((chunk_id, block_id)) = World::ids_at(neighbor_grid_position) {
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

        for (chunk_id, updates) in updates {
            if let Some(chunk) = self.get_chunk_mut(chunk_id) {
                for (block_id, visibility) in updates {
                    if let Some(meta) = chunk.get_meta_mut(block_id) {
                        meta.visibility = visibility;
                    }
                }
            }
        }
    }

    fn compute_visibility(&self, grid_position: IVec3) -> Vec<block::Direction> {
        let visibility: Vec<block::Direction> = Direction::faces()
            .iter()
            .filter_map(|&direction| {
                let neighbor_grid_position = grid_position + direction.offset();

                if self
                    .get_block(neighbor_grid_position)
                    .map_or(false, |block| block.kind == block::Kind::Air)
                {
                    Some(direction)
                } else {
                    None
                }
            })
            .collect();

        visibility
    }

    fn update_light(&mut self, _chunk_id: chunk::ID, _block_id: block::ID, _grid_position: IVec3) {}

    fn mark_update(&mut self, chunk_id: chunk::ID) {
        if let Some(chunk) = self.get_chunk_mut(chunk_id) {
            chunk.updated = true;
        }
    }

    fn update_chunk_mesh(&mut self, chunk_id: chunk::ID) {
        if let Some(chunk) = self.get_chunk(chunk_id) {
            if chunk.updated {
                let mesh = self.generate_chunk_mesh(chunk_id);

                if let Some(chunk) = self.get_chunk_mut(chunk_id) {
                    chunk.mesh = mesh;
                    chunk.updated = false;
                }
            }
        }
    }

    fn generate_chunk_mesh(&self, chunk_id: chunk::ID) -> chunk::Mesh {
        let mut faces = Vec::new();
        let chunk = self.get_chunk(chunk_id).unwrap();

        for block_id in (0..CHUNK_VOLUME).map(block::ID) {
            let meta = chunk.get_meta(block_id).unwrap();
            let block = chunk.get_block(block_id).unwrap();

            if block.solid {
                let grid_position = World::grid_position(chunk_id, block_id).unwrap();

                for direction in Direction::faces() {
                    if meta.visibility.contains(&direction) {
                        let mut face = Face::new(grid_position, direction, block.kind);

                        let (face_edges, face_corners) =
                            Self::get_face_neighbors(direction, &meta.neighbors);
                        face.light = Self::calculate_face_light(face_edges, face_corners);

                        faces.push(face);
                    }
                }
            }
        }

        chunk::Mesh { faces }
    }

    fn get_face_neighbors(
        direction: block::Direction,
        neighbors: &Vec<block::Direction>,
    ) -> ([bool; 4], [bool; 4]) {
        let face_neighbors = match direction {
            block::Direction::XpYoZo => (
                [
                    neighbors.contains(&block::Direction::XpYnZo),
                    neighbors.contains(&block::Direction::XpYoZn),
                    neighbors.contains(&block::Direction::XpYpZo),
                    neighbors.contains(&block::Direction::XpYoZp),
                ],
                [
                    neighbors.contains(&block::Direction::XpYnZn),
                    neighbors.contains(&block::Direction::XpYpZn),
                    neighbors.contains(&block::Direction::XpYpZp),
                    neighbors.contains(&block::Direction::XpYnZp),
                ],
            ),
            block::Direction::XnYoZo => (
                [
                    neighbors.contains(&block::Direction::XnYnZo),
                    neighbors.contains(&block::Direction::XnYoZp),
                    neighbors.contains(&block::Direction::XnYpZo),
                    neighbors.contains(&block::Direction::XnYoZn),
                ],
                [
                    neighbors.contains(&block::Direction::XnYnZp),
                    neighbors.contains(&block::Direction::XnYpZp),
                    neighbors.contains(&block::Direction::XnYpZn),
                    neighbors.contains(&block::Direction::XnYnZn),
                ],
            ),
            block::Direction::XoYpZo => (
                [
                    neighbors.contains(&block::Direction::XoYpZn),
                    neighbors.contains(&block::Direction::XnYpZo),
                    neighbors.contains(&block::Direction::XoYpZp),
                    neighbors.contains(&block::Direction::XpYpZo),
                ],
                [
                    neighbors.contains(&block::Direction::XnYpZn),
                    neighbors.contains(&block::Direction::XnYpZp),
                    neighbors.contains(&block::Direction::XpYpZp),
                    neighbors.contains(&block::Direction::XpYpZn),
                ],
            ),
            block::Direction::XoYnZo => (
                [
                    neighbors.contains(&block::Direction::XoYnZn),
                    neighbors.contains(&block::Direction::XpYnZo),
                    neighbors.contains(&block::Direction::XoYnZp),
                    neighbors.contains(&block::Direction::XnYnZo),
                ],
                [
                    neighbors.contains(&block::Direction::XpYnZn),
                    neighbors.contains(&block::Direction::XpYnZp),
                    neighbors.contains(&block::Direction::XnYnZp),
                    neighbors.contains(&block::Direction::XnYnZn),
                ],
            ),
            block::Direction::XoYoZp => (
                [
                    neighbors.contains(&block::Direction::XoYnZp),
                    neighbors.contains(&block::Direction::XpYoZp),
                    neighbors.contains(&block::Direction::XoYpZp),
                    neighbors.contains(&block::Direction::XnYoZp),
                ],
                [
                    neighbors.contains(&block::Direction::XpYnZp),
                    neighbors.contains(&block::Direction::XpYpZp),
                    neighbors.contains(&block::Direction::XnYpZp),
                    neighbors.contains(&block::Direction::XnYnZp),
                ],
            ),
            block::Direction::XoYoZn => (
                [
                    neighbors.contains(&block::Direction::XoYnZn),
                    neighbors.contains(&block::Direction::XnYoZn),
                    neighbors.contains(&block::Direction::XoYpZn),
                    neighbors.contains(&block::Direction::XpYoZn),
                ],
                [
                    neighbors.contains(&block::Direction::XnYnZn),
                    neighbors.contains(&block::Direction::XnYpZn),
                    neighbors.contains(&block::Direction::XpYpZn),
                    neighbors.contains(&block::Direction::XpYnZn),
                ],
            ),
            _ => panic!("Invalid Direction: {:?}", direction),
        };

        face_neighbors
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
            AMBIENT_LIGHT_LEVEL[0]
        } else if corner || edge1 || edge2 {
            AMBIENT_LIGHT_LEVEL[1]
        } else {
            AMBIENT_LIGHT_LEVEL[2]
        }
    }

    pub fn on_map(grid_position: IVec3) -> bool {
        let in_x_range = grid_position.x.abs() <= WORLD_BOUNDARY as i32;
        let in_y_range = grid_position.y.abs() <= WORLD_BOUNDARY as i32;
        let in_z_range = grid_position.z.abs() <= WORLD_BOUNDARY as i32;

        in_x_range && in_y_range && in_z_range
    }

    pub fn ids_at(grid_position: IVec3) -> Option<(chunk::ID, block::ID)> {
        let chunk_id = Chunk::id_at_grid(grid_position)?;
        let block_id = Block::id_at_grid(grid_position)?;

        Some((chunk_id, block_id))
    }

    pub fn grid_position(chunk_id: chunk::ID, block_id: block::ID) -> Option<IVec3> {
        let chunk_position = Chunk::position(chunk_id)?;
        let block_position = Block::position(block_id)?;

        let grid_position = CHUNK_SIZE as i32 * chunk_position + block_position;

        Some(grid_position)
    }

    pub fn position_at(grid_position: IVec3) -> Option<Vec3> {
        if Self::on_map(grid_position) {
            Some(grid_position.as_vec3())
        } else {
            None
        }
    }

    pub fn grid_position_at(position: Vec3) -> Option<IVec3> {
        let grid_position = position.as_ivec3();

        if Self::on_map(grid_position) {
            Some(grid_position)
        } else {
            None
        }
    }

    pub fn visible_chunk_ids(chunk_id: chunk::ID, radius: i32) -> Vec<chunk::ID> {
        let chunk_count_estimate = ((2 * radius + 1).pow(3)) as usize;
        let mut chunk_ids = Vec::with_capacity(chunk_count_estimate);

        if let Some(chunk_position) = Chunk::position(chunk_id) {
            for x in -radius..=radius {
                for y in -radius..=radius {
                    for z in -radius..=radius {
                        let distance = x.abs() + y.abs() + z.abs();

                        if distance <= radius {
                            let visible_chunk_position = chunk_position + IVec3::new(x, y, z);

                            if let Some(visible_chunk_id) = Chunk::id_at(visible_chunk_position) {
                                chunk_ids.push(visible_chunk_id);
                            }
                        }
                    }
                }
            }
        }

        chunk_ids
    }
}
