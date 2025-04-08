use crate::simulation::{
    block::{self, Direction, Face, Neighbors},
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
                tick: Tick::ZERO,
                updated: false,
                position: chunk_position,
                palette: Vec::from([block::Kind::Air]),
                blocks: Box::new([0; CHUNK_VOLUME]),
                meta: Box::new([block::Meta::default(); CHUNK_VOLUME]),
                light: Box::new([block::Light::default(); CHUNK_VOLUME]),
                mesh: chunk::Mesh::default(),
            }
        });

        chunks
    }

    pub fn generate(&mut self) {
        self.generate_ground();

        self.generate_structure(-12, 1, 12, &structure::Kind::Luigi);

        self.set_block_kind(0, 1, 0, &block::kind::Kind::Metal);
        self.set_block_kind(0, 2, 0, &block::kind::Kind::Metal);
        self.set_block_kind(0, 3, 0, &block::kind::Kind::GoldMetal);

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

                let (primary_color, secondary_color) =
                    if (chunk_position.x + chunk_position.z) % 2 == 0 {
                        (&block::Kind::GreenCloth, &block::Kind::Concrete)
                    } else {
                        (&block::Kind::BlueCloth, &block::Kind::Grey)
                    };

                let kind = if (x % 2 == 0) ^ (z % 2 == 0) {
                    primary_color
                } else {
                    secondary_color
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
            self.update_visibility(grid_position);
            self.update_light(chunk_id, block_id, grid_position);

            self.mark_update(chunk_id);

            true
        } else {
            false
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
        let mut updates: HashMap<chunk::ID, Vec<(block::ID, Neighbors)>> = HashMap::new();

        for offset in Direction::offsets() {
            let neighbor_grid_position = grid_position + offset;

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
        let mut updates: HashMap<chunk::ID, Vec<(block::ID, Face)>> = HashMap::new();

        if let Some((chunk_id, block_id)) = World::ids_at(grid_position) {
            let visibility = self.compute_visibility(grid_position);

            updates
                .entry(chunk_id)
                .or_insert_with(Vec::new)
                .push((block_id, visibility));
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
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let chunk = self.get_chunk(chunk_id).unwrap();

        for block_id in (0..CHUNK_VOLUME).map(block::ID) {
            let meta = chunk.get_meta(block_id).unwrap();
            let block = chunk.get_block(block_id).unwrap();
            let grid_position = World::grid_position(chunk_id, block_id).unwrap();

            for face in block::Face::ALL {
                if meta.visibility.contains(face) == false {
                    continue;
                }

                let face_quad = Self::generate_quad(grid_position, face);
                let normal = face.normal();

                let color = Vec4::new(block.color.0, block.color.1, block.color.2, block.color.3);

                let (face_edges, face_corners) = Self::get_face_neighbors(meta.neighbors, face);
                let face_light = Self::calculate_face_light(face_edges, face_corners);

                let chunk_vertices = face_quad.iter().enumerate().map(|(index, &position)| {
                    let light = face_light[index];

                    chunk::Vertex {
                        position,
                        normal,
                        color,
                        light,
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

    fn get_face_neighbors(
        neighbors: block::Neighbors,
        face: block::Face,
    ) -> ([bool; 4], [bool; 4]) {
        let face_neighbors = match face {
            block::Face::XP => (
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
            block::Face::XN => (
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
            block::Face::YP => (
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
            block::Face::YN => (
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
            block::Face::ZP => (
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
            block::Face::ZN => (
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
        };

        face_neighbors
    }

    fn calculate_face_light(edges: [bool; 4], corners: [bool; 4]) -> [f32; 4] {
        [
            Self::calculate_vertex_light(edges[3], edges[0], corners[3]),
            Self::calculate_vertex_light(edges[0], edges[1], corners[0]),
            Self::calculate_vertex_light(edges[1], edges[2], corners[1]),
            Self::calculate_vertex_light(edges[2], edges[3], corners[2]),
        ]
    }

    fn calculate_vertex_light(edge1: bool, edge2: bool, corner: bool) -> f32 {
        if edge1 && edge2 {
            chunk::Vertex::LIGHT_LEVEL[0]
        } else if corner || edge1 || edge2 {
            chunk::Vertex::LIGHT_LEVEL[1]
        } else {
            chunk::Vertex::LIGHT_LEVEL[2]
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
}
