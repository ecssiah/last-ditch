use glam::{IVec3, Vec3};
use last_ditch::simulation::{
    world::{block, chunk, World},
    TEST_CHUNK_RADIUS, TEST_WORLD_RADIUS,
};

struct WorldToGridTestCase {
    description: String,
    world_position: Vec3,
    expected_grid_position: Option<IVec3>,
}

impl WorldToGridTestCase {
    pub fn check(&self, world: &World) {
        let grid_position = world.grid.world_to_grid(self.world_position);

        assert_eq!(
            grid_position, self.expected_grid_position,
            "{:?}",
            self.description,
        );
    }
}

#[test]
fn world_to_grid() {
    let test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);

    let boundary = test_world.grid.boundary as f32;

    let test_cases = vec![
        WorldToGridTestCase {
            description: "".to_string(),
            world_position: Vec3::new(0.0, 0.0, 0.0),
            expected_grid_position: Some(IVec3::new(0, 0, 0)),
        },
        WorldToGridTestCase {
            description: "".to_string(),
            world_position: Vec3::new(boundary, boundary, boundary),
            expected_grid_position: Some(IVec3::new(
                boundary as i32,
                boundary as i32,
                boundary as i32,
            )),
        },
        WorldToGridTestCase {
            description: "".to_string(),
            world_position: Vec3::new(boundary + 1.0, boundary + 1.0, boundary + 1.0),
            expected_grid_position: None,
        },
        WorldToGridTestCase {
            description: "".to_string(),
            world_position: Vec3::new(-boundary, -boundary, -boundary),
            expected_grid_position: Some(IVec3::new(
                -boundary as i32,
                -boundary as i32,
                -boundary as i32,
            )),
        },
        WorldToGridTestCase {
            description: "".to_string(),
            world_position: Vec3::new(-boundary - 1.0, -boundary - 1.0, -boundary - 1.0),
            expected_grid_position: None,
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}

struct GridToChunkTestCase {
    description: String,
    grid_position: IVec3,
    expected_chunk_position: Option<IVec3>,
}

impl GridToChunkTestCase {
    pub fn check(&self, world: &World) {
        let chunk_position = world.grid.grid_to_chunk(self.grid_position);

        assert_eq!(
            chunk_position, self.expected_chunk_position,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn grid_to_chunk() {
    let test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);

    let boundary = test_world.grid.boundary as i32;
    let radius = test_world.grid.radius as i32;

    let test_cases = vec![
        GridToChunkTestCase {
            description: "".to_string(),
            grid_position: IVec3::new(0, 0, 0),
            expected_chunk_position: Some(IVec3::new(0, 0, 0)),
        },
        GridToChunkTestCase {
            description: "".to_string(),
            grid_position: IVec3::new(boundary, boundary, boundary),
            expected_chunk_position: Some(IVec3::new(radius, radius, radius)),
        },
        GridToChunkTestCase {
            description: "".to_string(),
            grid_position: IVec3::new(boundary + 1, boundary + 1, boundary + 1),
            expected_chunk_position: None,
        },
        GridToChunkTestCase {
            description: "".to_string(),
            grid_position: IVec3::new(-boundary, -boundary, -boundary),
            expected_chunk_position: Some(IVec3::new(-radius, -radius, -radius)),
        },
        GridToChunkTestCase {
            description: "".to_string(),
            grid_position: IVec3::new(-boundary - 1, -boundary - 1, -boundary - 1),
            expected_chunk_position: None,
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}

struct GridToBlockTestCase {
    description: String,
    grid_position: IVec3,
    expected_block_position: Option<IVec3>,
}

impl GridToBlockTestCase {
    pub fn check(&self, world: &World) {
        let block_position = world.grid.grid_to_block(self.grid_position);

        assert_eq!(
            block_position, self.expected_block_position,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn grid_to_block() {
    let test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);

    let boundary = test_world.grid.boundary as i32;
    let chunk_radius = test_world.grid.chunk_radius as i32;

    let test_cases = vec![
        GridToBlockTestCase {
            description: String::from("origin"),
            grid_position: IVec3::new(0, 0, 0),
            expected_block_position: Some(IVec3::new(0, 0, 0)),
        },
        GridToBlockTestCase {
            description: String::from("maximum position"),
            grid_position: IVec3::new(boundary, boundary, boundary),
            expected_block_position: Some(IVec3::new(chunk_radius, chunk_radius, chunk_radius)),
        },
        GridToBlockTestCase {
            description: String::from("beyond maximum position"),
            grid_position: IVec3::new(boundary + 1, boundary + 1, boundary + 1),
            expected_block_position: None,
        },
        GridToBlockTestCase {
            description: String::from("minimum position"),
            grid_position: IVec3::new(-boundary, -boundary, -boundary),
            expected_block_position: Some(IVec3::new(-chunk_radius, -chunk_radius, -chunk_radius)),
        },
        GridToBlockTestCase {
            description: String::from("beyond minimum position"),
            grid_position: IVec3::new(-boundary - 1, -boundary - 1, -boundary - 1),
            expected_block_position: None,
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}

struct ChunkToGridTestCase {
    description: String,
    chunk_position: IVec3,
    expected_grid_position: Option<IVec3>,
}

impl ChunkToGridTestCase {
    pub fn check(&self, world: &World) {
        let grid_position = world.grid.chunk_to_grid(self.chunk_position);

        assert_eq!(
            grid_position, self.expected_grid_position,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn chunk_to_grid() {
    let test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);

    let radius = test_world.grid.radius as i32;
    let chunk_size = test_world.grid.chunk_size as i32;

    let test_cases = vec![
        ChunkToGridTestCase {
            description: "".to_string(),
            chunk_position: IVec3::new(0, 0, 0),
            expected_grid_position: Some(IVec3::new(0, 0, 0)),
        },
        ChunkToGridTestCase {
            description: "".to_string(),
            chunk_position: IVec3::new(radius, radius, radius),
            expected_grid_position: Some(IVec3::new(
                radius * chunk_size,
                radius * chunk_size,
                radius * chunk_size,
            )),
        },
        ChunkToGridTestCase {
            description: "".to_string(),
            chunk_position: IVec3::new(radius + 1, radius + 1, radius + 1),
            expected_grid_position: None,
        },
        ChunkToGridTestCase {
            description: "".to_string(),
            chunk_position: IVec3::new(-radius, -radius, -radius),
            expected_grid_position: Some(IVec3::new(
                -radius * chunk_size,
                -radius * chunk_size,
                -radius * chunk_size,
            )),
        },
        ChunkToGridTestCase {
            description: "".to_string(),
            chunk_position: IVec3::new(-radius - 1, -radius - 1, -radius - 1),
            expected_grid_position: None,
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}

struct GridToChunkIDTestCase {
    description: String,
    grid_position: IVec3,
    expected_chunk_id: Option<chunk::ID>,
}

impl GridToChunkIDTestCase {
    pub fn check(&self, world: &World) {
        let chunk_id = world.grid.grid_to_chunk_id(self.grid_position);

        assert_eq!(chunk_id, self.expected_chunk_id, "{:?}", self.description);
    }
}

#[test]
fn grid_to_chunk_id() {
    let test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);

    let boundary = test_world.grid.boundary as i32;
    let volume = test_world.grid.volume;

    let test_cases = vec![
        GridToChunkIDTestCase {
            description: "".to_string(),
            grid_position: IVec3::new(0, 0, 0),
            expected_chunk_id: Some(chunk::ID((volume - 1) / 2)),
        },
        GridToChunkIDTestCase {
            description: "".to_string(),
            grid_position: IVec3::new(boundary, boundary, boundary),
            expected_chunk_id: Some(chunk::ID(volume - 1)),
        },
        GridToChunkIDTestCase {
            description: "".to_string(),
            grid_position: IVec3::new(boundary + 1, boundary + 1, boundary + 1),
            expected_chunk_id: None,
        },
        GridToChunkIDTestCase {
            description: "".to_string(),
            grid_position: IVec3::new(-boundary, -boundary, -boundary),
            expected_chunk_id: Some(chunk::ID(0)),
        },
        GridToChunkIDTestCase {
            description: "".to_string(),
            grid_position: IVec3::new(-boundary - 1, -boundary - 1, -boundary - 1),
            expected_chunk_id: None,
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}

struct WorldToChunkIDTestCase {
    description: String,
    world_position: Vec3,
    expected_chunk_id: Option<chunk::ID>,
}

impl WorldToChunkIDTestCase {
    pub fn check(&self, world: &World) {
        let chunk_id = world.grid.world_to_chunk_id(self.world_position);

        assert_eq!(chunk_id, self.expected_chunk_id, "{:?}", self.description);
    }
}

#[test]
fn world_to_chunk_id() {
    let test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);

    let boundary = test_world.grid.boundary as f32;
    let volume = test_world.grid.volume;

    let test_cases = vec![
        WorldToChunkIDTestCase {
            description: "".to_string(),
            world_position: Vec3::new(0.0, 0.0, 0.0),
            expected_chunk_id: Some(chunk::ID((volume - 1) / 2)),
        },
        WorldToChunkIDTestCase {
            description: "".to_string(),
            world_position: Vec3::new(boundary, boundary, boundary),
            expected_chunk_id: Some(chunk::ID(volume - 1)),
        },
        WorldToChunkIDTestCase {
            description: "".to_string(),
            world_position: Vec3::new(boundary + 1.0, boundary + 1.0, boundary + 1.0),
            expected_chunk_id: None,
        },
        WorldToChunkIDTestCase {
            description: "".to_string(),
            world_position: Vec3::new(-boundary, -boundary, -boundary),
            expected_chunk_id: Some(chunk::ID(0)),
        },
        WorldToChunkIDTestCase {
            description: "".to_string(),
            world_position: Vec3::new(-boundary - 1.0, -boundary - 1.0, -boundary - 1.0),
            expected_chunk_id: None,
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}

struct ChunkIDToPositionTestCase {
    description: String,
    chunk_id: chunk::ID,
    expected_chunk_position: Option<IVec3>,
}

impl ChunkIDToPositionTestCase {
    pub fn check(&self, world: &World) {
        let chunk_position = world.grid.chunk_id_to_position(self.chunk_id);

        assert_eq!(
            chunk_position, self.expected_chunk_position,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn chunk_id_to_position() {
    let test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);

    let radius = test_world.grid.radius as i32;

    let test_cases = vec![
        ChunkIDToPositionTestCase {
            description: "".to_string(),
            chunk_id: chunk::ID(0),
            expected_chunk_position: Some(IVec3::new(-radius, -radius, -radius)),
        },
        ChunkIDToPositionTestCase {
            description: "".to_string(),
            chunk_id: chunk::ID(test_world.grid.chunk_id_max / 2),
            expected_chunk_position: Some(IVec3::new(0, 0, 0)),
        },
        ChunkIDToPositionTestCase {
            description: "".to_string(),
            chunk_id: chunk::ID(test_world.grid.chunk_id_max),
            expected_chunk_position: Some(IVec3::new(radius, radius, radius)),
        },
        ChunkIDToPositionTestCase {
            description: "".to_string(),
            chunk_id: chunk::ID(test_world.grid.chunk_id_max + 1),
            expected_chunk_position: None,
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}

struct GridToBlockIDTestCase {
    description: String,
    grid_position: IVec3,
    expected_block_id: Option<block::ID>,
}

impl GridToBlockIDTestCase {
    pub fn check(&self, world: &World) {
        let block_id = world.grid.grid_to_block_id(self.grid_position);

        assert_eq!(block_id, self.expected_block_id, "{:?}", self.description);
    }
}

#[test]
fn grid_to_block_id() {
    let test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);

    let chunk_radius = test_world.grid.chunk_radius as i32;
    let chunk_size = test_world.grid.chunk_size as i32;
    let boundary = test_world.grid.boundary as i32;

    let test_cases = vec![
        GridToBlockIDTestCase {
            description: "".to_string(),
            grid_position: IVec3::new(0, 0, 0),
            expected_block_id: Some(block::ID(test_world.grid.block_id_max / 2)),
        },
        GridToBlockIDTestCase {
            description: "".to_string(),
            grid_position: IVec3::new(-boundary - 1, -boundary - 1, -boundary - 1),
            expected_block_id: None,
        },
        GridToBlockIDTestCase {
            description: "".to_string(),
            grid_position: IVec3::new(boundary + 1, boundary + 1, boundary + 1),
            expected_block_id: None,
        },
        GridToBlockIDTestCase {
            description: "".to_string(),
            grid_position: IVec3::new(chunk_size, chunk_size, chunk_size),
            expected_block_id: Some(block::ID(test_world.grid.block_id_max / 2)),
        },
        GridToBlockIDTestCase {
            description: "".to_string(),
            grid_position: IVec3::new(-chunk_size, -chunk_size, -chunk_size),
            expected_block_id: Some(block::ID(test_world.grid.block_id_max / 2)),
        },
        GridToBlockIDTestCase {
            description: "".to_string(),
            grid_position: IVec3::new(chunk_radius, chunk_radius, chunk_radius),
            expected_block_id: Some(block::ID(test_world.grid.block_id_max)),
        },
        GridToBlockIDTestCase {
            description: "".to_string(),
            grid_position: IVec3::new(-chunk_radius, -chunk_radius, -chunk_radius),
            expected_block_id: Some(block::ID(0)),
        },
        GridToBlockIDTestCase {
            description: "".to_string(),
            grid_position: IVec3::new(
                chunk_size + chunk_radius,
                chunk_size + chunk_radius,
                chunk_size + chunk_radius,
            ),
            expected_block_id: Some(block::ID(test_world.grid.block_id_max)),
        },
        GridToBlockIDTestCase {
            description: "".to_string(),
            grid_position: IVec3::new(
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

struct BlockIDToPositionTestCase {
    description: String,
    block_id: block::ID,
    expected_block_position: Option<IVec3>,
}

impl BlockIDToPositionTestCase {
    pub fn check(&self, world: &World) {
        let block_position = world.grid.block_id_to_position(self.block_id);

        assert_eq!(
            block_position, self.expected_block_position,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn block_id_to_position() {
    let test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);

    let chunk_radius = test_world.grid.chunk_radius as i32;

    let test_cases = vec![
        BlockIDToPositionTestCase {
            description: "".to_string(),
            block_id: block::ID(test_world.grid.block_id_max / 2),
            expected_block_position: Some(IVec3::new(0, 0, 0)),
        },
        BlockIDToPositionTestCase {
            description: "".to_string(),
            block_id: block::ID(test_world.grid.block_id_max + 1),
            expected_block_position: None,
        },
        BlockIDToPositionTestCase {
            description: "".to_string(),
            block_id: block::ID(0),
            expected_block_position: Some(IVec3::new(-chunk_radius, -chunk_radius, -chunk_radius)),
        },
        BlockIDToPositionTestCase {
            description: "".to_string(),
            block_id: block::ID(test_world.grid.block_id_max),
            expected_block_position: Some(IVec3::new(chunk_radius, chunk_radius, chunk_radius)),
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}

struct GridToIDsTestCase {
    description: String,
    grid_position: IVec3,
    expected_chunk_id: chunk::ID,
    expected_block_id: block::ID,
}

impl GridToIDsTestCase {
    pub fn check(&self, world: &World) {
        let ids = world.grid.grid_to_ids(self.grid_position);

        assert!(ids.is_some(), "{:?}", self.description);

        let (chunk_id, block_id) = ids.unwrap();

        assert_eq!(chunk_id, self.expected_chunk_id, "{:?}", self.description);

        assert_eq!(block_id, self.expected_block_id, "{:?}", self.description);
    }
}

#[test]
fn grid_to_ids() {
    let test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);

    let radius = test_world.grid.radius as i32;
    let size = test_world.grid.size as i32;
    let chunk_radius = test_world.grid.chunk_radius as i32;
    let chunk_size = test_world.grid.chunk_size as i32;

    let boundary = test_world.grid.boundary as i32;

    let test_cases = vec![
        GridToIDsTestCase {
            description: "".to_string(),
            grid_position: IVec3::new(0, 0, 0),
            expected_chunk_id: chunk::ID(test_world.grid.chunk_id_max / 2),
            expected_block_id: block::ID(test_world.grid.block_id_max / 2),
        },
        GridToIDsTestCase {
            description: "".to_string(),
            grid_position: IVec3::new(boundary, boundary, boundary),
            expected_chunk_id: chunk::ID(test_world.grid.chunk_id_max),
            expected_block_id: block::ID(test_world.grid.block_id_max),
        },
        GridToIDsTestCase {
            description: "".to_string(),
            grid_position: IVec3::new(-boundary, -boundary, -boundary),
            expected_chunk_id: chunk::ID(0),
            expected_block_id: block::ID(0),
        },
        GridToIDsTestCase {
            description: "".to_string(),
            grid_position: IVec3::new(
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
        GridToIDsTestCase {
            description: "".to_string(),
            grid_position: IVec3::new(
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

struct IDsToGridTestCase {
    description: String,
    chunk_id: chunk::ID,
    block_id: block::ID,
    expected_grid_position: Option<IVec3>,
}

impl IDsToGridTestCase {
    pub fn check(&self, world: &World) {
        let grid_position = world.grid.ids_to_grid(self.chunk_id, self.block_id);

        assert_eq!(
            grid_position, self.expected_grid_position,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn ids_to_grid() {
    let test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);

    let radius = test_world.grid.radius as i32;
    let size = test_world.grid.size as i32;
    let chunk_radius = test_world.grid.chunk_radius as i32;
    let chunk_size = test_world.grid.chunk_size as i32;

    let boundary = test_world.grid.boundary as i32;

    let test_cases = vec![
        IDsToGridTestCase {
            description: "ids at (0, 0, 0)".to_string(),
            chunk_id: chunk::ID(test_world.grid.chunk_id_max / 2),
            block_id: block::ID(test_world.grid.block_id_max / 2),
            expected_grid_position: Some(IVec3::new(0, 0, 0)),
        },
        IDsToGridTestCase {
            description: String::from("ids at (boundary, boundary, boundary)"),
            chunk_id: chunk::ID(test_world.grid.chunk_id_max),
            block_id: block::ID(test_world.grid.block_id_max),
            expected_grid_position: Some(IVec3::new(boundary, boundary, boundary)),
        },
        IDsToGridTestCase {
            description: "ids at (-boundary, -boundary, -boundary)".to_string(),
            chunk_id: chunk::ID(0),
            block_id: block::ID(0),
            expected_grid_position: Some(IVec3::new(-boundary, -boundary, -boundary)),
        },
        IDsToGridTestCase {
            description: "ids at minimum of chunk (-1, -1, -1)".to_string(),
            chunk_id: {
                let chunk_position = IVec3::new(-1, -1, -1);

                let chunk_index = (chunk_position.x + radius)
                    + (chunk_position.y + radius) * size
                    + (chunk_position.z + radius) * size * size;

                chunk::ID(chunk_index as u32)
            },
            block_id: block::ID(0),
            expected_grid_position: Some(IVec3::new(
                -chunk_size - chunk_radius,
                -chunk_size - chunk_radius,
                -chunk_size - chunk_radius,
            )),
        },
        IDsToGridTestCase {
            description: "ids at max of chunk (1, 1, 1)".to_string(),
            chunk_id: {
                let chunk_position = IVec3::new(1, 1, 1);

                let chunk_index = (chunk_position.x + radius)
                    + (chunk_position.y + radius) * size
                    + (chunk_position.z + radius) * size * size;

                chunk::ID(chunk_index as u32)
            },
            block_id: block::ID(test_world.grid.block_id_max),
            expected_grid_position: Some(IVec3::new(
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
