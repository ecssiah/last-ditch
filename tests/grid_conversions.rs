use glam::{IVec3, Vec3};
use last_ditch::simulation::world::{block, chunk, World};

struct WorldToGridTestCase {
    description: String,
    world_position: Vec3,
    expected_grid_position: Option<IVec3>,
}

#[test]
fn world_to_grid() {
    let test_world = World::new(1, 2);

    let boundary = test_world.grid.boundary as f32;

    let test_cases = vec![
        WorldToGridTestCase {
            description: String::from(""),
            world_position: Vec3::new(0.0, 0.0, 0.0),
            expected_grid_position: Some(IVec3::new(0, 0, 0)),
        },
        WorldToGridTestCase {
            description: String::from(""),
            world_position: Vec3::new(boundary, boundary, boundary),
            expected_grid_position: Some(IVec3::new(
                boundary as i32,
                boundary as i32,
                boundary as i32,
            )),
        },
        WorldToGridTestCase {
            description: String::from(""),
            world_position: Vec3::new(boundary + 1.0, boundary + 1.0, boundary + 1.0),
            expected_grid_position: None,
        },
        WorldToGridTestCase {
            description: String::from(""),
            world_position: Vec3::new(-boundary, -boundary, -boundary),
            expected_grid_position: Some(IVec3::new(
                -boundary as i32,
                -boundary as i32,
                -boundary as i32,
            )),
        },
        WorldToGridTestCase {
            description: String::from(""),
            world_position: Vec3::new(-boundary - 1.0, -boundary - 1.0, -boundary - 1.0),
            expected_grid_position: None,
        },
    ];

    for test_case in test_cases {
        let grid_position = test_world.grid.world_to_grid(test_case.world_position);

        assert_eq!(
            grid_position, test_case.expected_grid_position,
            "{:?}",
            test_case.description,
        );
    }
}

struct GridToChunkTestCase {
    description: String,
    grid_position: IVec3,
    expected_chunk_position: Option<IVec3>,
}

#[test]
fn grid_to_chunk() {
    let test_world = World::new(1, 2);

    let boundary = test_world.grid.boundary as i32;
    let radius = test_world.grid.radius as i32;

    let test_cases = vec![
        GridToChunkTestCase {
            description: String::from(""),
            grid_position: IVec3::new(0, 0, 0),
            expected_chunk_position: Some(IVec3::new(0, 0, 0)),
        },
        GridToChunkTestCase {
            description: String::from(""),
            grid_position: IVec3::new(boundary, boundary, boundary),
            expected_chunk_position: Some(IVec3::new(radius, radius, radius)),
        },
        GridToChunkTestCase {
            description: String::from(""),
            grid_position: IVec3::new(boundary + 1, boundary + 1, boundary + 1),
            expected_chunk_position: None,
        },
        GridToChunkTestCase {
            description: String::from(""),
            grid_position: IVec3::new(-boundary, -boundary, -boundary),
            expected_chunk_position: Some(IVec3::new(-radius, -radius, -radius)),
        },
        GridToChunkTestCase {
            description: String::from(""),
            grid_position: IVec3::new(-boundary - 1, -boundary - 1, -boundary - 1),
            expected_chunk_position: None,
        },
    ];

    for test_case in test_cases {
        let chunk_position = test_world.grid.grid_to_chunk(test_case.grid_position);

        assert_eq!(
            chunk_position, test_case.expected_chunk_position,
            "{:?}",
            test_case.description
        );
    }
}

struct GridToBlockTestCase {
    description: String,
    grid_position: IVec3,
    expected_block_position: Option<IVec3>,
}

#[test]
fn grid_to_block() {
    let test_world = World::new(1, 2);

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
        let block_position = test_world.grid.grid_to_block(test_case.grid_position);

        assert_eq!(
            block_position, test_case.expected_block_position,
            "{:?}",
            test_case.description
        );
    }
}

struct ChunkToGridTestCase {
    description: String,
    chunk_position: IVec3,
    expected_grid_position: Option<IVec3>,
}

#[test]
fn chunk_to_grid() {
    let test_world = World::new(1, 2);

    let radius = test_world.grid.radius as i32;
    let chunk_size = test_world.grid.chunk_size as i32;

    let test_cases = vec![
        ChunkToGridTestCase {
            description: String::from(""),
            chunk_position: IVec3::new(0, 0, 0),
            expected_grid_position: Some(IVec3::new(0, 0, 0)),
        },
        ChunkToGridTestCase {
            description: String::from(""),
            chunk_position: IVec3::new(radius, radius, radius),
            expected_grid_position: Some(IVec3::new(
                radius * chunk_size,
                radius * chunk_size,
                radius * chunk_size,
            )),
        },
        ChunkToGridTestCase {
            description: String::from(""),
            chunk_position: IVec3::new(radius + 1, radius + 1, radius + 1),
            expected_grid_position: None,
        },
        ChunkToGridTestCase {
            description: String::from(""),
            chunk_position: IVec3::new(-radius, -radius, -radius),
            expected_grid_position: Some(IVec3::new(
                -radius * chunk_size,
                -radius * chunk_size,
                -radius * chunk_size,
            )),
        },
        ChunkToGridTestCase {
            description: String::from(""),
            chunk_position: IVec3::new(-radius - 1, -radius - 1, -radius - 1),
            expected_grid_position: None,
        },
    ];

    for test_case in test_cases {
        let grid_position = test_world.grid.chunk_to_grid(test_case.chunk_position);

        assert_eq!(
            grid_position, test_case.expected_grid_position,
            "{:?}",
            test_case.description
        );
    }
}

struct GridToChunkIDTestCase {
    description: String,
    grid_position: IVec3,
    expected_chunk_id: Option<chunk::ID>,
}

#[test]
fn grid_to_chunk_id() {
    let test_world = World::new(1, 2);

    let boundary = test_world.grid.boundary as i32;
    let volume = test_world.grid.volume;

    let test_cases = vec![
        GridToChunkIDTestCase {
            description: String::from(""),
            grid_position: IVec3::new(0, 0, 0),
            expected_chunk_id: Some(chunk::ID((volume - 1) / 2)),
        },
        GridToChunkIDTestCase {
            description: String::from(""),
            grid_position: IVec3::new(boundary, boundary, boundary),
            expected_chunk_id: Some(chunk::ID(volume - 1)),
        },
        GridToChunkIDTestCase {
            description: String::from(""),
            grid_position: IVec3::new(boundary + 1, boundary + 1, boundary + 1),
            expected_chunk_id: None,
        },
        GridToChunkIDTestCase {
            description: String::from(""),
            grid_position: IVec3::new(-boundary, -boundary, -boundary),
            expected_chunk_id: Some(chunk::ID(0)),
        },
        GridToChunkIDTestCase {
            description: String::from(""),
            grid_position: IVec3::new(-boundary - 1, -boundary - 1, -boundary - 1),
            expected_chunk_id: None,
        },
    ];

    for test_case in test_cases {
        let chunk_id = test_world.grid.grid_to_chunk_id(test_case.grid_position);

        assert_eq!(
            chunk_id, test_case.expected_chunk_id,
            "{:?}",
            test_case.description
        );
    }
}

struct WorldToChunkIDTestCase {
    description: String,
    world_position: Vec3,
    expected_chunk_id: Option<chunk::ID>,
}

#[test]
fn world_to_chunk_id() {
    let test_world = World::new(1, 2);

    let boundary = test_world.grid.boundary as f32;
    let volume = test_world.grid.volume;

    let test_cases = vec![
        WorldToChunkIDTestCase {
            description: String::from(""),
            world_position: Vec3::new(0.0, 0.0, 0.0),
            expected_chunk_id: Some(chunk::ID((volume - 1) / 2)),
        },
        WorldToChunkIDTestCase {
            description: String::from(""),
            world_position: Vec3::new(boundary, boundary, boundary),
            expected_chunk_id: Some(chunk::ID(volume - 1)),
        },
        WorldToChunkIDTestCase {
            description: String::from(""),
            world_position: Vec3::new(boundary + 1.0, boundary + 1.0, boundary + 1.0),
            expected_chunk_id: None,
        },
        WorldToChunkIDTestCase {
            description: String::from(""),
            world_position: Vec3::new(-boundary, -boundary, -boundary),
            expected_chunk_id: Some(chunk::ID(0)),
        },
        WorldToChunkIDTestCase {
            description: String::from(""),
            world_position: Vec3::new(-boundary - 1.0, -boundary - 1.0, -boundary - 1.0),
            expected_chunk_id: None,
        },
    ];

    for test_case in test_cases {
        let chunk_id = test_world.grid.world_to_chunk_id(test_case.world_position);

        assert_eq!(
            chunk_id, test_case.expected_chunk_id,
            "{:?}",
            test_case.description
        );
    }
}

struct ChunkIDToPositionTestCase {
    description: String,
    chunk_id: chunk::ID,
    expected_chunk_position: Option<IVec3>,
}

#[test]
fn chunk_id_to_position() {
    let test_world = World::new(1, 2);

    let radius = test_world.grid.radius as i32;

    let test_cases = vec![
        ChunkIDToPositionTestCase {
            description: String::from(""),
            chunk_id: chunk::ID(0),
            expected_chunk_position: Some(IVec3::new(-radius, -radius, -radius)),
        },
        ChunkIDToPositionTestCase {
            description: String::from(""),
            chunk_id: chunk::ID(test_world.grid.chunk_id_max / 2),
            expected_chunk_position: Some(IVec3::new(0, 0, 0)),
        },
        ChunkIDToPositionTestCase {
            description: String::from(""),
            chunk_id: chunk::ID(test_world.grid.chunk_id_max),
            expected_chunk_position: Some(IVec3::new(radius, radius, radius)),
        },
        ChunkIDToPositionTestCase {
            description: String::from(""),
            chunk_id: chunk::ID(test_world.grid.chunk_id_max + 1),
            expected_chunk_position: None,
        },
    ];

    for test_case in test_cases {
        let chunk_position = test_world.grid.chunk_id_to_position(test_case.chunk_id);

        assert_eq!(
            chunk_position, test_case.expected_chunk_position,
            "{:?}",
            test_case.description
        );
    }
}

struct GridToBlockIDTestCase {
    description: String,
    grid_position: IVec3,
    expected_block_id: Option<block::ID>,
}

#[test]
fn grid_to_block_id() {
    let test_world = World::new(1, 2);

    let chunk_radius = test_world.grid.chunk_radius as i32;
    let chunk_size = test_world.grid.chunk_size as i32;
    let boundary = test_world.grid.boundary as i32;

    let test_cases = vec![
        GridToBlockIDTestCase {
            description: String::from(""),
            grid_position: IVec3::new(0, 0, 0),
            expected_block_id: Some(block::ID(test_world.grid.block_id_max / 2)),
        },
        GridToBlockIDTestCase {
            description: String::from(""),
            grid_position: IVec3::new(-boundary - 1, -boundary - 1, -boundary - 1),
            expected_block_id: None,
        },
        GridToBlockIDTestCase {
            description: String::from(""),
            grid_position: IVec3::new(boundary + 1, boundary + 1, boundary + 1),
            expected_block_id: None,
        },
        GridToBlockIDTestCase {
            description: String::from(""),
            grid_position: IVec3::new(chunk_size, chunk_size, chunk_size),
            expected_block_id: Some(block::ID(test_world.grid.block_id_max / 2)),
        },
        GridToBlockIDTestCase {
            description: String::from(""),
            grid_position: IVec3::new(-chunk_size, -chunk_size, -chunk_size),
            expected_block_id: Some(block::ID(test_world.grid.block_id_max / 2)),
        },
        GridToBlockIDTestCase {
            description: String::from(""),
            grid_position: IVec3::new(chunk_radius, chunk_radius, chunk_radius),
            expected_block_id: Some(block::ID(test_world.grid.block_id_max)),
        },
        GridToBlockIDTestCase {
            description: String::from(""),
            grid_position: IVec3::new(-chunk_radius, -chunk_radius, -chunk_radius),
            expected_block_id: Some(block::ID(0)),
        },
        GridToBlockIDTestCase {
            description: String::from(""),
            grid_position: IVec3::new(
                chunk_size + chunk_radius,
                chunk_size + chunk_radius,
                chunk_size + chunk_radius,
            ),
            expected_block_id: Some(block::ID(test_world.grid.block_id_max)),
        },
        GridToBlockIDTestCase {
            description: String::from(""),
            grid_position: IVec3::new(
                -chunk_size - chunk_radius,
                -chunk_size - chunk_radius,
                -chunk_size - chunk_radius,
            ),
            expected_block_id: Some(block::ID(0)),
        },
    ];

    for test_case in test_cases {
        let block_id = test_world.grid.grid_to_block_id(test_case.grid_position);

        assert_eq!(
            block_id, test_case.expected_block_id,
            "{:?}",
            test_case.description
        );
    }
}

struct BlockIDToPositionTestCase {
    description: String,
    block_id: block::ID,
    expected_block_position: Option<IVec3>,
}

#[test]
fn block_id_to_position() {
    let test_world = World::new(1, 2);

    let chunk_radius = test_world.grid.chunk_radius as i32;

    let test_cases = vec![
        BlockIDToPositionTestCase {
            description: String::from(""),
            block_id: block::ID(test_world.grid.block_id_max / 2),
            expected_block_position: Some(IVec3::new(0, 0, 0)),
        },
        BlockIDToPositionTestCase {
            description: String::from(""),
            block_id: block::ID(test_world.grid.block_id_max + 1),
            expected_block_position: None,
        },
        BlockIDToPositionTestCase {
            description: String::from(""),
            block_id: block::ID(0),
            expected_block_position: Some(IVec3::new(-chunk_radius, -chunk_radius, -chunk_radius)),
        },
        BlockIDToPositionTestCase {
            description: String::from(""),
            block_id: block::ID(test_world.grid.block_id_max),
            expected_block_position: Some(IVec3::new(chunk_radius, chunk_radius, chunk_radius)),
        },
    ];

    for test_case in test_cases {
        let block_position = test_world.grid.block_id_to_position(test_case.block_id);

        assert_eq!(
            block_position, test_case.expected_block_position,
            "{:?}",
            test_case.description
        );
    }
}

struct GridToIDsTestCase {
    description: String,
    grid_position: IVec3,
    expected_chunk_id: chunk::ID,
    expected_block_id: block::ID,
}

#[test]
fn grid_to_ids() {
    let test_world = World::new(1, 2);

    let radius = test_world.grid.radius as i32;
    let size = test_world.grid.size as i32;
    let chunk_radius = test_world.grid.chunk_radius as i32;
    let chunk_size = test_world.grid.chunk_size as i32;

    let boundary = test_world.grid.boundary as i32;

    let test_cases = vec![
        GridToIDsTestCase {
            description: String::from(""),
            grid_position: IVec3::new(0, 0, 0),
            expected_chunk_id: chunk::ID(test_world.grid.chunk_id_max / 2),
            expected_block_id: block::ID(test_world.grid.block_id_max / 2),
        },
        GridToIDsTestCase {
            description: String::from(""),
            grid_position: IVec3::new(boundary, boundary, boundary),
            expected_chunk_id: chunk::ID(test_world.grid.chunk_id_max),
            expected_block_id: block::ID(test_world.grid.block_id_max),
        },
        GridToIDsTestCase {
            description: String::from(""),
            grid_position: IVec3::new(-boundary, -boundary, -boundary),
            expected_chunk_id: chunk::ID(0),
            expected_block_id: block::ID(0),
        },
        GridToIDsTestCase {
            description: String::from(""),
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
            description: String::from(""),
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
        let id_result = test_world.grid.grid_to_ids(test_case.grid_position);

        assert!(id_result.is_some(), "{:?}", test_case.description);

        let (chunk_id, block_id) = id_result.unwrap();

        assert_eq!(
            chunk_id, test_case.expected_chunk_id,
            "{:?}",
            test_case.description
        );

        assert_eq!(
            block_id, test_case.expected_block_id,
            "{:?}",
            test_case.description
        );
    }
}

struct IDsToGridTestCase {
    description: String,
    chunk_id: chunk::ID,
    block_id: block::ID,
    expected_grid_position: Option<IVec3>,
}

#[test]
fn ids_to_grid() {
    let test_world = World::new(1, 2);

    let radius = test_world.grid.radius as i32;
    let size = test_world.grid.size as i32;
    let chunk_radius = test_world.grid.chunk_radius as i32;
    let chunk_size = test_world.grid.chunk_size as i32;

    let boundary = test_world.grid.boundary as i32;

    let test_cases = vec![
        IDsToGridTestCase {
            description: String::from("ids at (0, 0, 0)"),
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
            description: String::from("ids at (-boundary, -boundary, -boundary)"),
            chunk_id: chunk::ID(0),
            block_id: block::ID(0),
            expected_grid_position: Some(IVec3::new(-boundary, -boundary, -boundary)),
        },
        IDsToGridTestCase {
            description: String::from("ids at minimum of chunk (-1, -1, -1)"),
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
            description: String::from("ids at max of chunk (1, 1, 1)"),
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
        let grid_position = test_world
            .grid
            .ids_to_grid(test_case.chunk_id, test_case.block_id);

        assert_eq!(
            grid_position, test_case.expected_grid_position,
            "{:?}",
            test_case.description
        );
    }
}
