use crate::simulation::{
    consts::*,
    world::{block, chunk, World},
};
use glam::{IVec3, Vec3};

struct WorldToGridCase {
    description: String,
    world_position: Vec3,
    expected_position: Option<IVec3>,
}

impl WorldToGridCase {
    pub fn check(&self, world: &World) {
        let position = world.grid.world_to_grid(self.world_position);

        assert_eq!(
            position, self.expected_position,
            "{:?}",
            self.description,
        );
    }
}

#[test]
fn world_to_grid() {
    let test_world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

    let boundary = test_world.grid.boundary as f32;

    let test_cases = vec![
        WorldToGridCase {
            description: "".to_string(),
            world_position: Vec3::new(0.0, 0.0, 0.0),
            expected_position: Some(IVec3::new(0, 0, 0)),
        },
        WorldToGridCase {
            description: "".to_string(),
            world_position: Vec3::new(boundary, boundary, boundary),
            expected_position: Some(IVec3::new(
                boundary as i32,
                boundary as i32,
                boundary as i32,
            )),
        },
        WorldToGridCase {
            description: "".to_string(),
            world_position: Vec3::new(boundary + 1.0, boundary + 1.0, boundary + 1.0),
            expected_position: None,
        },
        WorldToGridCase {
            description: "".to_string(),
            world_position: Vec3::new(-boundary, -boundary, -boundary),
            expected_position: Some(IVec3::new(
                -boundary as i32,
                -boundary as i32,
                -boundary as i32,
            )),
        },
        WorldToGridCase {
            description: "".to_string(),
            world_position: Vec3::new(-boundary - 1.0, -boundary - 1.0, -boundary - 1.0),
            expected_position: None,
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}

struct GridToChunkCase {
    description: String,
    position: IVec3,
    expected_chunk_position: Option<IVec3>,
}

impl GridToChunkCase {
    pub fn check(&self, world: &World) {
        let chunk_position = world.grid.grid_to_chunk(self.position);

        assert_eq!(
            chunk_position, self.expected_chunk_position,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn grid_to_chunk() {
    let test_world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

    let boundary = test_world.grid.boundary as i32;
    let radius = test_world.grid.radius as i32;

    let test_cases = vec![
        GridToChunkCase {
            description: "".to_string(),
            position: IVec3::new(0, 0, 0),
            expected_chunk_position: Some(IVec3::new(0, 0, 0)),
        },
        GridToChunkCase {
            description: "".to_string(),
            position: IVec3::new(boundary, boundary, boundary),
            expected_chunk_position: Some(IVec3::new(radius, radius, radius)),
        },
        GridToChunkCase {
            description: "".to_string(),
            position: IVec3::new(boundary + 1, boundary + 1, boundary + 1),
            expected_chunk_position: None,
        },
        GridToChunkCase {
            description: "".to_string(),
            position: IVec3::new(-boundary, -boundary, -boundary),
            expected_chunk_position: Some(IVec3::new(-radius, -radius, -radius)),
        },
        GridToChunkCase {
            description: "".to_string(),
            position: IVec3::new(-boundary - 1, -boundary - 1, -boundary - 1),
            expected_chunk_position: None,
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}

struct GridToBlockCase {
    description: String,
    position: IVec3,
    expected_block_position: Option<IVec3>,
}

impl GridToBlockCase {
    pub fn check(&self, world: &World) {
        let block_position = world.grid.grid_to_block(self.position);

        assert_eq!(
            block_position, self.expected_block_position,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn grid_to_block() {
    let test_world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

    let boundary = test_world.grid.boundary as i32;
    let chunk_radius = test_world.grid.chunk_radius as i32;

    let test_cases = vec![
        GridToBlockCase {
            description: String::from("origin"),
            position: IVec3::new(0, 0, 0),
            expected_block_position: Some(IVec3::new(0, 0, 0)),
        },
        GridToBlockCase {
            description: String::from("maximum position"),
            position: IVec3::new(boundary, boundary, boundary),
            expected_block_position: Some(IVec3::new(chunk_radius, chunk_radius, chunk_radius)),
        },
        GridToBlockCase {
            description: String::from("beyond maximum position"),
            position: IVec3::new(boundary + 1, boundary + 1, boundary + 1),
            expected_block_position: None,
        },
        GridToBlockCase {
            description: String::from("minimum position"),
            position: IVec3::new(-boundary, -boundary, -boundary),
            expected_block_position: Some(IVec3::new(-chunk_radius, -chunk_radius, -chunk_radius)),
        },
        GridToBlockCase {
            description: String::from("beyond minimum position"),
            position: IVec3::new(-boundary - 1, -boundary - 1, -boundary - 1),
            expected_block_position: None,
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}

struct ChunkToGridCase {
    description: String,
    chunk_position: IVec3,
    expected_position: Option<IVec3>,
}

impl ChunkToGridCase {
    pub fn check(&self, world: &World) {
        let position = world.grid.chunk_to_grid(self.chunk_position);

        assert_eq!(
            position, self.expected_position,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn chunk_to_grid() {
    let test_world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

    let radius = test_world.grid.radius as i32;
    let chunk_size = test_world.grid.chunk_size as i32;

    let test_cases = vec![
        ChunkToGridCase {
            description: "".to_string(),
            chunk_position: IVec3::new(0, 0, 0),
            expected_position: Some(IVec3::new(0, 0, 0)),
        },
        ChunkToGridCase {
            description: "".to_string(),
            chunk_position: IVec3::new(radius, radius, radius),
            expected_position: Some(IVec3::new(
                radius * chunk_size,
                radius * chunk_size,
                radius * chunk_size,
            )),
        },
        ChunkToGridCase {
            description: "".to_string(),
            chunk_position: IVec3::new(radius + 1, radius + 1, radius + 1),
            expected_position: None,
        },
        ChunkToGridCase {
            description: "".to_string(),
            chunk_position: IVec3::new(-radius, -radius, -radius),
            expected_position: Some(IVec3::new(
                -radius * chunk_size,
                -radius * chunk_size,
                -radius * chunk_size,
            )),
        },
        ChunkToGridCase {
            description: "".to_string(),
            chunk_position: IVec3::new(-radius - 1, -radius - 1, -radius - 1),
            expected_position: None,
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}

struct GridToChunkIDCase {
    description: String,
    position: IVec3,
    expected_chunk_id: Option<chunk::ID>,
}

impl GridToChunkIDCase {
    pub fn check(&self, world: &World) {
        let chunk_id = world.grid.grid_to_chunk_id(self.position);

        assert_eq!(chunk_id, self.expected_chunk_id, "{:?}", self.description);
    }
}

#[test]
fn grid_to_chunk_id() {
    let test_world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

    let boundary = test_world.grid.boundary as i32;
    let volume = test_world.grid.volume;

    let test_cases = vec![
        GridToChunkIDCase {
            description: "".to_string(),
            position: IVec3::new(0, 0, 0),
            expected_chunk_id: Some(chunk::ID((volume - 1) / 2)),
        },
        GridToChunkIDCase {
            description: "".to_string(),
            position: IVec3::new(boundary, boundary, boundary),
            expected_chunk_id: Some(chunk::ID(volume - 1)),
        },
        GridToChunkIDCase {
            description: "".to_string(),
            position: IVec3::new(boundary + 1, boundary + 1, boundary + 1),
            expected_chunk_id: None,
        },
        GridToChunkIDCase {
            description: "".to_string(),
            position: IVec3::new(-boundary, -boundary, -boundary),
            expected_chunk_id: Some(chunk::ID(0)),
        },
        GridToChunkIDCase {
            description: "".to_string(),
            position: IVec3::new(-boundary - 1, -boundary - 1, -boundary - 1),
            expected_chunk_id: None,
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}

struct WorldToChunkIDCase {
    description: String,
    world_position: Vec3,
    expected_chunk_id: Option<chunk::ID>,
}

impl WorldToChunkIDCase {
    pub fn check(&self, world: &World) {
        let chunk_id = world.grid.world_to_chunk_id(self.world_position);

        assert_eq!(chunk_id, self.expected_chunk_id, "{:?}", self.description);
    }
}

#[test]
fn world_to_chunk_id() {
    let test_world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

    let boundary = test_world.grid.boundary as f32;
    let volume = test_world.grid.volume;

    let test_cases = vec![
        WorldToChunkIDCase {
            description: "".to_string(),
            world_position: Vec3::new(0.0, 0.0, 0.0),
            expected_chunk_id: Some(chunk::ID((volume - 1) / 2)),
        },
        WorldToChunkIDCase {
            description: "".to_string(),
            world_position: Vec3::new(boundary, boundary, boundary),
            expected_chunk_id: Some(chunk::ID(volume - 1)),
        },
        WorldToChunkIDCase {
            description: "".to_string(),
            world_position: Vec3::new(boundary + 1.0, boundary + 1.0, boundary + 1.0),
            expected_chunk_id: None,
        },
        WorldToChunkIDCase {
            description: "".to_string(),
            world_position: Vec3::new(-boundary, -boundary, -boundary),
            expected_chunk_id: Some(chunk::ID(0)),
        },
        WorldToChunkIDCase {
            description: "".to_string(),
            world_position: Vec3::new(-boundary - 1.0, -boundary - 1.0, -boundary - 1.0),
            expected_chunk_id: None,
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}

struct ChunkIDToPositionCase {
    description: String,
    chunk_id: chunk::ID,
    expected_chunk_position: Option<IVec3>,
}

impl ChunkIDToPositionCase {
    pub fn check(&self, world: &World) {
        let chunk_position = world.grid.chunk_id_to_chunk_position(self.chunk_id);

        assert_eq!(
            chunk_position, self.expected_chunk_position,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn chunk_id_to_chunk_position() {
    let test_world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

    let radius = test_world.grid.radius as i32;

    let test_cases = vec![
        ChunkIDToPositionCase {
            description: "".to_string(),
            chunk_id: chunk::ID(0),
            expected_chunk_position: Some(IVec3::new(-radius, -radius, -radius)),
        },
        ChunkIDToPositionCase {
            description: "".to_string(),
            chunk_id: chunk::ID(test_world.grid.chunk_id_max / 2),
            expected_chunk_position: Some(IVec3::new(0, 0, 0)),
        },
        ChunkIDToPositionCase {
            description: "".to_string(),
            chunk_id: chunk::ID(test_world.grid.chunk_id_max),
            expected_chunk_position: Some(IVec3::new(radius, radius, radius)),
        },
        ChunkIDToPositionCase {
            description: "".to_string(),
            chunk_id: chunk::ID(test_world.grid.chunk_id_max + 1),
            expected_chunk_position: None,
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}

struct GridToBlockIDCase {
    description: String,
    position: IVec3,
    expected_block_id: Option<block::ID>,
}

impl GridToBlockIDCase {
    pub fn check(&self, world: &World) {
        let block_id = world.grid.grid_to_block_id(self.position);

        assert_eq!(block_id, self.expected_block_id, "{:?}", self.description);
    }
}

#[test]
fn grid_to_block_id() {
    let test_world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

    let chunk_radius = test_world.grid.chunk_radius as i32;
    let chunk_size = test_world.grid.chunk_size as i32;
    let boundary = test_world.grid.boundary as i32;

    let test_cases = vec![
        GridToBlockIDCase {
            description: "".to_string(),
            position: IVec3::new(0, 0, 0),
            expected_block_id: Some(block::ID(test_world.grid.block_id_max / 2)),
        },
        GridToBlockIDCase {
            description: "".to_string(),
            position: IVec3::new(-boundary - 1, -boundary - 1, -boundary - 1),
            expected_block_id: None,
        },
        GridToBlockIDCase {
            description: "".to_string(),
            position: IVec3::new(boundary + 1, boundary + 1, boundary + 1),
            expected_block_id: None,
        },
        GridToBlockIDCase {
            description: "".to_string(),
            position: IVec3::new(chunk_size, chunk_size, chunk_size),
            expected_block_id: Some(block::ID(test_world.grid.block_id_max / 2)),
        },
        GridToBlockIDCase {
            description: "".to_string(),
            position: IVec3::new(-chunk_size, -chunk_size, -chunk_size),
            expected_block_id: Some(block::ID(test_world.grid.block_id_max / 2)),
        },
        GridToBlockIDCase {
            description: "".to_string(),
            position: IVec3::new(chunk_radius, chunk_radius, chunk_radius),
            expected_block_id: Some(block::ID(test_world.grid.block_id_max)),
        },
        GridToBlockIDCase {
            description: "".to_string(),
            position: IVec3::new(-chunk_radius, -chunk_radius, -chunk_radius),
            expected_block_id: Some(block::ID(0)),
        },
        GridToBlockIDCase {
            description: "".to_string(),
            position: IVec3::new(
                chunk_size + chunk_radius,
                chunk_size + chunk_radius,
                chunk_size + chunk_radius,
            ),
            expected_block_id: Some(block::ID(test_world.grid.block_id_max)),
        },
        GridToBlockIDCase {
            description: "".to_string(),
            position: IVec3::new(
                -chunk_size - chunk_radius,
                -chunk_size - chunk_radius,
                -chunk_size - chunk_radius,
            ),
            expected_block_id: Some(block::ID(0)),
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}

struct BlockIDToPositionCase {
    description: String,
    block_id: block::ID,
    expected_block_position: Option<IVec3>,
}

impl BlockIDToPositionCase {
    pub fn check(&self, world: &World) {
        let block_position = world.grid.block_id_to_block_position(self.block_id);

        assert_eq!(
            block_position, self.expected_block_position,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn block_id_to_block_position() {
    let test_world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

    let chunk_radius = test_world.grid.chunk_radius as i32;

    let test_cases = vec![
        BlockIDToPositionCase {
            description: "".to_string(),
            block_id: block::ID(test_world.grid.block_id_max / 2),
            expected_block_position: Some(IVec3::new(0, 0, 0)),
        },
        BlockIDToPositionCase {
            description: "".to_string(),
            block_id: block::ID(test_world.grid.block_id_max + 1),
            expected_block_position: None,
        },
        BlockIDToPositionCase {
            description: "".to_string(),
            block_id: block::ID(0),
            expected_block_position: Some(IVec3::new(-chunk_radius, -chunk_radius, -chunk_radius)),
        },
        BlockIDToPositionCase {
            description: "".to_string(),
            block_id: block::ID(test_world.grid.block_id_max),
            expected_block_position: Some(IVec3::new(chunk_radius, chunk_radius, chunk_radius)),
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}

struct GridToIDsCase {
    description: String,
    position: IVec3,
    expected_chunk_id: chunk::ID,
    expected_block_id: block::ID,
}

impl GridToIDsCase {
    pub fn check(&self, world: &World) {
        let ids = world.grid.grid_to_ids(self.position);

        assert!(ids.is_some(), "{:?}", self.description);

        let (chunk_id, block_id) = ids.unwrap();

        assert_eq!(chunk_id, self.expected_chunk_id, "{:?}", self.description);

        assert_eq!(block_id, self.expected_block_id, "{:?}", self.description);
    }
}

#[test]
fn grid_to_ids() {
    let test_world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

    let radius = test_world.grid.radius as i32;
    let size = test_world.grid.size as i32;
    let chunk_radius = test_world.grid.chunk_radius as i32;
    let chunk_size = test_world.grid.chunk_size as i32;

    let boundary = test_world.grid.boundary as i32;

    let test_cases = vec![
        GridToIDsCase {
            description: "".to_string(),
            position: IVec3::new(0, 0, 0),
            expected_chunk_id: chunk::ID(test_world.grid.chunk_id_max / 2),
            expected_block_id: block::ID(test_world.grid.block_id_max / 2),
        },
        GridToIDsCase {
            description: "".to_string(),
            position: IVec3::new(boundary, boundary, boundary),
            expected_chunk_id: chunk::ID(test_world.grid.chunk_id_max),
            expected_block_id: block::ID(test_world.grid.block_id_max),
        },
        GridToIDsCase {
            description: "".to_string(),
            position: IVec3::new(-boundary, -boundary, -boundary),
            expected_chunk_id: chunk::ID(0),
            expected_block_id: block::ID(0),
        },
        GridToIDsCase {
            description: "".to_string(),
            position: IVec3::new(
                -chunk_size - chunk_radius,
                -chunk_size - chunk_radius,
                -chunk_size - chunk_radius,
            ),
            expected_chunk_id: {
                let chunk_position = IVec3::new(-1, -1, -1);

                let chunk_index = (chunk_position.x + radius)
                    + (chunk_position.y + radius) * size
                    + (chunk_position.z + radius) * size * size;

                chunk::ID(chunk_index as u32)
            },
            expected_block_id: block::ID(0),
        },
        GridToIDsCase {
            description: "".to_string(),
            position: IVec3::new(
                chunk_size + chunk_radius,
                chunk_size + chunk_radius,
                chunk_size + chunk_radius,
            ),
            expected_chunk_id: {
                let chunk_position = IVec3::new(1, 1, 1);

                let chunk_index = (chunk_position.x + radius)
                    + (chunk_position.y + radius) * size
                    + (chunk_position.z + radius) * size * size;

                chunk::ID(chunk_index as u32)
            },
            expected_block_id: block::ID(test_world.grid.block_id_max),
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}

struct IDsToGridCase {
    description: String,
    chunk_id: chunk::ID,
    block_id: block::ID,
    expected_position: Option<IVec3>,
}

impl IDsToGridCase {
    pub fn check(&self, world: &World) {
        let position = world.grid.ids_to_position(self.chunk_id, self.block_id);

        assert_eq!(
            position, self.expected_position,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn ids_to_grid() {
    let test_world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

    let radius = test_world.grid.radius as i32;
    let size = test_world.grid.size as i32;
    let chunk_radius = test_world.grid.chunk_radius as i32;
    let chunk_size = test_world.grid.chunk_size as i32;

    let boundary = test_world.grid.boundary as i32;

    let test_cases = vec![
        IDsToGridCase {
            description: "ids at (0, 0, 0)".to_string(),
            chunk_id: chunk::ID(test_world.grid.chunk_id_max / 2),
            block_id: block::ID(test_world.grid.block_id_max / 2),
            expected_position: Some(IVec3::new(0, 0, 0)),
        },
        IDsToGridCase {
            description: String::from("ids at (boundary, boundary, boundary)"),
            chunk_id: chunk::ID(test_world.grid.chunk_id_max),
            block_id: block::ID(test_world.grid.block_id_max),
            expected_position: Some(IVec3::new(boundary, boundary, boundary)),
        },
        IDsToGridCase {
            description: "ids at (-boundary, -boundary, -boundary)".to_string(),
            chunk_id: chunk::ID(0),
            block_id: block::ID(0),
            expected_position: Some(IVec3::new(-boundary, -boundary, -boundary)),
        },
        IDsToGridCase {
            description: "ids at minimum of chunk (-1, -1, -1)".to_string(),
            chunk_id: {
                let chunk_position = IVec3::new(-1, -1, -1);

                let chunk_index = (chunk_position.x + radius)
                    + (chunk_position.y + radius) * size
                    + (chunk_position.z + radius) * size * size;

                chunk::ID(chunk_index as u32)
            },
            block_id: block::ID(0),
            expected_position: Some(IVec3::new(
                -chunk_size - chunk_radius,
                -chunk_size - chunk_radius,
                -chunk_size - chunk_radius,
            )),
        },
        IDsToGridCase {
            description: "ids at max of chunk (1, 1, 1)".to_string(),
            chunk_id: {
                let chunk_position = IVec3::new(1, 1, 1);

                let chunk_index = (chunk_position.x + radius)
                    + (chunk_position.y + radius) * size
                    + (chunk_position.z + radius) * size * size;

                chunk::ID(chunk_index as u32)
            },
            block_id: block::ID(test_world.grid.block_id_max),
            expected_position: Some(IVec3::new(
                chunk_size + chunk_radius,
                chunk_size + chunk_radius,
                chunk_size + chunk_radius,
            )),
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}
