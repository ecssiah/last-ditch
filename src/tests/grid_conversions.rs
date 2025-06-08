use crate::simulation::{
    consts::*,
    world::{block, builder, chunk, World},
};
use glam::IVec3;

struct BlockIDToBlockCoordinatesCase {
    description: String,
    block_id: block::ID,
    expected_block_coordinates: IVec3,
}

impl BlockIDToBlockCoordinatesCase {
    pub fn check(&self, world: &World) {
        let block_coordinates = world.grid.block_id_to_block_coordinates(self.block_id);

        assert!(block_coordinates.is_some(), "{:?}", self.description);

        let block_coordinates = block_coordinates.unwrap();

        assert_eq!(
            block_coordinates, self.expected_block_coordinates,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn block_id_to_block_coordinates() {
    let mut world = World::new(TEST_CHUNK_RADIUS, TEST_WORLD_RADIUS);

    builder::TestWorld::build(&mut world);

    let test_cases = vec![
        BlockIDToBlockCoordinatesCase {
            description: "block id min".to_string(),
            block_id: block::ID(0),
            expected_block_coordinates: IVec3::new(
                -(world.grid.chunk_radius as i32),
                -(world.grid.chunk_radius as i32),
                -(world.grid.chunk_radius as i32),
            ),
        },
        BlockIDToBlockCoordinatesCase {
            description: "block id at origin".to_string(),
            block_id: block::ID(world.grid.block_index_max / 2),
            expected_block_coordinates: IVec3::new(0, 0, 0),
        },
        BlockIDToBlockCoordinatesCase {
            description: "block id max".to_string(),
            block_id: block::ID(world.grid.block_index_max),
            expected_block_coordinates: IVec3::new(
                world.grid.chunk_radius as i32,
                world.grid.chunk_radius as i32,
                world.grid.chunk_radius as i32,
            ),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct BlockCoordinatesToBlockIDCase {
    description: String,
    block_coordinates: IVec3,
    expected_block_id: block::ID,
}

impl BlockCoordinatesToBlockIDCase {
    pub fn check(&self, world: &World) {
        let block_id = world
            .grid
            .block_coordinates_to_block_id(self.block_coordinates);

        assert!(block_id.is_some(), "{:?}", self.description);

        let block_id = block_id.unwrap();

        assert_eq!(block_id, self.expected_block_id, "{:?}", self.description);
    }
}

#[test]
fn block_coordinates_to_block_id() {
    let mut world = World::new(TEST_CHUNK_RADIUS, TEST_WORLD_RADIUS);

    builder::TestWorld::build(&mut world);

    let test_cases = vec![
        BlockCoordinatesToBlockIDCase {
            description: "block coordinates min".to_string(),
            block_coordinates: IVec3::new(
                -(world.grid.chunk_radius as i32),
                -(world.grid.chunk_radius as i32),
                -(world.grid.chunk_radius as i32),
            ),
            expected_block_id: block::ID(0),
        },
        BlockCoordinatesToBlockIDCase {
            description: "block coordinates origin".to_string(),
            block_coordinates: IVec3::new(0, 0, 0),
            expected_block_id: block::ID(world.grid.block_index_max / 2),
        },
        BlockCoordinatesToBlockIDCase {
            description: "block coordinates max".to_string(),
            block_coordinates: IVec3::new(
                world.grid.chunk_radius as i32,
                world.grid.chunk_radius as i32,
                world.grid.chunk_radius as i32,
            ),
            expected_block_id: block::ID(world.grid.block_index_max),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct ChunkIDToChunkCoordinates {
    description: String,
    chunk_id: chunk::ID,
    expected_chunk_coordinates: IVec3,
}

impl ChunkIDToChunkCoordinates {
    pub fn check(&self, world: &World) {
        let chunk_coordinates = world.grid.chunk_id_to_chunk_coordinates(self.chunk_id);

        assert!(chunk_coordinates.is_some(), "{:?}", self.description);

        let chunk_coordinates = chunk_coordinates.unwrap();

        assert_eq!(
            chunk_coordinates, self.expected_chunk_coordinates,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn chunk_id_to_chunk_coordinates() {
    let mut world = World::new(TEST_CHUNK_RADIUS, TEST_WORLD_RADIUS);

    builder::TestWorld::build(&mut world);

    let test_cases = vec![
        ChunkIDToChunkCoordinates {
            description: "chunk id min".to_string(),
            chunk_id: chunk::ID(0),
            expected_chunk_coordinates: IVec3::new(
                -(world.grid.world_radius as i32),
                -(world.grid.world_radius as i32),
                -(world.grid.world_radius as i32),
            ),
        },
        ChunkIDToChunkCoordinates {
            description: "chunk id at origin".to_string(),
            chunk_id: chunk::ID(world.grid.chunk_index_max / 2),
            expected_chunk_coordinates: IVec3::new(0, 0, 0),
        },
        ChunkIDToChunkCoordinates {
            description: "chunk id max".to_string(),
            chunk_id: chunk::ID(world.grid.chunk_index_max),
            expected_chunk_coordinates: IVec3::new(
                world.grid.world_radius as i32,
                world.grid.world_radius as i32,
                world.grid.world_radius as i32,
            ),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct ChunkCoordinatesToChunkIDCase {
    description: String,
    chunk_coordinates: IVec3,
    expected_chunk_id: chunk::ID,
}

impl ChunkCoordinatesToChunkIDCase {
    pub fn check(&self, world: &World) {
        let chunk_id = world
            .grid
            .chunk_coordinates_to_chunk_id(self.chunk_coordinates);

        assert!(chunk_id.is_some(), "{:?}", self.description);

        let chunk_id = chunk_id.unwrap();

        assert_eq!(chunk_id, self.expected_chunk_id, "{:?}", self.description);
    }
}

#[test]
fn chunk_coordinates_to_chunk_id() {
    let mut world = World::new(TEST_CHUNK_RADIUS, TEST_WORLD_RADIUS);

    builder::TestWorld::build(&mut world);

    let test_cases = vec![
        ChunkCoordinatesToChunkIDCase {
            description: "chunk coordinates min".to_string(),
            chunk_coordinates: IVec3::new(
                -(world.grid.world_radius as i32),
                -(world.grid.world_radius as i32),
                -(world.grid.world_radius as i32),
            ),
            expected_chunk_id: chunk::ID(0),
        },
        ChunkCoordinatesToChunkIDCase {
            description: "chunk coordinates origin".to_string(),
            chunk_coordinates: IVec3::new(0, 0, 0),
            expected_chunk_id: chunk::ID(world.grid.chunk_index_max / 2),
        },
        ChunkCoordinatesToChunkIDCase {
            description: "chunk coordinates max".to_string(),
            chunk_coordinates: IVec3::new(
                world.grid.world_radius as i32,
                world.grid.world_radius as i32,
                world.grid.world_radius as i32,
            ),
            expected_chunk_id: chunk::ID(world.grid.chunk_index_max),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}









// struct WorldToPositionCase {
//     description: String,
//     world_position: Vec3,
//     expected_position: Option<IVec3>,
// }

// impl WorldToPositionCase {
//     pub fn check(&self, world: &World) {
//         let position = world.grid.world_to_position(self.world_position);

//         assert_eq!(position, self.expected_position, "{:?}", self.description,);
//     }
// }

// #[test]
// fn world_to_grid() {
//     let test_world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

//     let world_boundary = test_world.grid.world_boundary as f32;

//     let test_cases = vec![
//         WorldToPositionCase {
//             description: "".to_string(),
//             world_position: Vec3::new(0.0, 0.0, 0.0),
//             expected_position: Some(IVec3::new(0, 0, 0)),
//         },
//         WorldToPositionCase {
//             description: "".to_string(),
//             world_position: Vec3::new(world_boundary, world_boundary, world_boundary),
//             expected_position: Some(IVec3::new(
//                 world_boundary as i32,
//                 world_boundary as i32,
//                 world_boundary as i32,
//             )),
//         },
//         WorldToPositionCase {
//             description: "".to_string(),
//             world_position: Vec3::new(
//                 world_boundary + 1.0,
//                 world_boundary + 1.0,
//                 world_boundary + 1.0,
//             ),
//             expected_position: None,
//         },
//         WorldToPositionCase {
//             description: "".to_string(),
//             world_position: Vec3::new(-world_boundary, -world_boundary, -world_boundary),
//             expected_position: Some(IVec3::new(
//                 -world_boundary as i32,
//                 -world_boundary as i32,
//                 -world_boundary as i32,
//             )),
//         },
//         WorldToPositionCase {
//             description: "".to_string(),
//             world_position: Vec3::new(
//                 -world_boundary - 1.0,
//                 -world_boundary - 1.0,
//                 -world_boundary - 1.0,
//             ),
//             expected_position: None,
//         },
//     ];

//     for test_case in test_cases {
//         test_case.check(&test_world);
//     }
// }

// struct GridToChunkCase {
//     description: String,
//     position: IVec3,
//     expected_chunk_coordinates: Option<IVec3>,
// }

// impl GridToChunkCase {
//     pub fn check(&self, world: &World) {
//         let chunk_coordinates = world.grid.position_to_chunk_coordinates(self.position);

//         assert_eq!(
//             chunk_coordinates, self.expected_chunk_coordinates,
//             "{:?}",
//             self.description
//         );
//     }
// }

// #[test]
// fn grid_to_chunk() {
//     let test_world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

//     let world_boundary = test_world.grid.world_boundary as i32;
//     let world_radius = test_world.grid.world_radius as i32;

//     let test_cases = vec![
//         GridToChunkCase {
//             description: "".to_string(),
//             position: IVec3::new(0, 0, 0),
//             expected_chunk_coordinates: Some(IVec3::new(0, 0, 0)),
//         },
//         GridToChunkCase {
//             description: "".to_string(),
//             position: IVec3::new(world_boundary, world_boundary, world_boundary),
//             expected_chunk_coordinates: Some(IVec3::new(world_radius, world_radius, world_radius)),
//         },
//         GridToChunkCase {
//             description: "".to_string(),
//             position: IVec3::new(world_boundary + 1, world_boundary + 1, world_boundary + 1),
//             expected_chunk_coordinates: None,
//         },
//         GridToChunkCase {
//             description: "".to_string(),
//             position: IVec3::new(-world_boundary, -world_boundary, -world_boundary),
//             expected_chunk_coordinates: Some(IVec3::new(
//                 -world_radius,
//                 -world_radius,
//                 -world_radius,
//             )),
//         },
//         GridToChunkCase {
//             description: "".to_string(),
//             position: IVec3::new(
//                 -world_boundary - 1,
//                 -world_boundary - 1,
//                 -world_boundary - 1,
//             ),
//             expected_chunk_coordinates: None,
//         },
//     ];

//     for test_case in test_cases {
//         test_case.check(&test_world);
//     }
// }

// struct GridToBlockCase {
//     description: String,
//     position: IVec3,
//     expected_block_coordinates: Option<IVec3>,
// }

// impl GridToBlockCase {
//     pub fn check(&self, world: &World) {
//         let block_coordinates = world.grid.position_to_block_coordinates(self.position);

//         assert_eq!(
//             block_coordinates, self.expected_block_coordinates,
//             "{:?}",
//             self.description
//         );
//     }
// }

// #[test]
// fn grid_to_block() {
//     let test_world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

//     let world_boundary = test_world.grid.world_boundary as i32;
//     let chunk_radius = test_world.grid.chunk_radius as i32;

//     let test_cases = vec![
//         GridToBlockCase {
//             description: String::from("origin"),
//             position: IVec3::new(0, 0, 0),
//             expected_block_coordinates: Some(IVec3::new(0, 0, 0)),
//         },
//         GridToBlockCase {
//             description: String::from("maximum position"),
//             position: IVec3::new(world_boundary, world_boundary, world_boundary),
//             expected_block_coordinates: Some(IVec3::new(chunk_radius, chunk_radius, chunk_radius)),
//         },
//         GridToBlockCase {
//             description: String::from("beyond maximum position"),
//             position: IVec3::new(world_boundary + 1, world_boundary + 1, world_boundary + 1),
//             expected_block_coordinates: None,
//         },
//         GridToBlockCase {
//             description: String::from("minimum position"),
//             position: IVec3::new(-world_boundary, -world_boundary, -world_boundary),
//             expected_block_coordinates: Some(IVec3::new(
//                 -chunk_radius,
//                 -chunk_radius,
//                 -chunk_radius,
//             )),
//         },
//         GridToBlockCase {
//             description: String::from("beyond minimum position"),
//             position: IVec3::new(
//                 -world_boundary - 1,
//                 -world_boundary - 1,
//                 -world_boundary - 1,
//             ),
//             expected_block_coordinates: None,
//         },
//     ];

//     for test_case in test_cases {
//         test_case.check(&test_world);
//     }
// }

// struct ChunkToGridCase {
//     description: String,
//     chunk_coordinates: IVec3,
//     expected_position: Option<IVec3>,
// }

// impl ChunkToGridCase {
//     pub fn check(&self, world: &World) {
//         let position = world
//             .grid
//             .chunk_coordinates_to_position(self.chunk_coordinates);

//         assert_eq!(position, self.expected_position, "{:?}", self.description);
//     }
// }

// #[test]
// fn chunk_to_grid() {
//     let test_world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

//     let world_radius = test_world.grid.world_radius as i32;
//     let chunk_size = test_world.grid.chunk_size as i32;

//     let test_cases = vec![
//         ChunkToGridCase {
//             description: "".to_string(),
//             chunk_coordinates: IVec3::new(0, 0, 0),
//             expected_position: Some(IVec3::new(0, 0, 0)),
//         },
//         ChunkToGridCase {
//             description: "".to_string(),
//             chunk_coordinates: IVec3::new(world_radius, world_radius, world_radius),
//             expected_position: Some(IVec3::new(
//                 world_radius * chunk_size,
//                 world_radius * chunk_size,
//                 world_radius * chunk_size,
//             )),
//         },
//         ChunkToGridCase {
//             description: "".to_string(),
//             chunk_coordinates: IVec3::new(world_radius + 1, world_radius + 1, world_radius + 1),
//             expected_position: None,
//         },
//         ChunkToGridCase {
//             description: "".to_string(),
//             chunk_coordinates: IVec3::new(-world_radius, -world_radius, -world_radius),
//             expected_position: Some(IVec3::new(
//                 -world_radius * chunk_size,
//                 -world_radius * chunk_size,
//                 -world_radius * chunk_size,
//             )),
//         },
//         ChunkToGridCase {
//             description: "".to_string(),
//             chunk_coordinates: IVec3::new(-world_radius - 1, -world_radius - 1, -world_radius - 1),
//             expected_position: None,
//         },
//     ];

//     for test_case in test_cases {
//         test_case.check(&test_world);
//     }
// }

// struct GridToChunkIDCase {
//     description: String,
//     position: IVec3,
//     expected_chunk_id: Option<chunk::ID>,
// }

// impl GridToChunkIDCase {
//     pub fn check(&self, world: &World) {
//         let chunk_id = world.grid.position_to_chunk_id(self.position);

//         assert_eq!(chunk_id, self.expected_chunk_id, "{:?}", self.description);
//     }
// }

// #[test]
// fn grid_to_chunk_id() {
//     let test_world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

//     let world_boundary = test_world.grid.world_boundary as i32;
//     let world_volume = test_world.grid.world_volume;

//     let test_cases = vec![
//         GridToChunkIDCase {
//             description: "".to_string(),
//             position: IVec3::new(0, 0, 0),
//             expected_chunk_id: Some(chunk::ID((world_volume - 1) / 2)),
//         },
//         GridToChunkIDCase {
//             description: "".to_string(),
//             position: IVec3::new(world_boundary, world_boundary, world_boundary),
//             expected_chunk_id: Some(chunk::ID(world_volume - 1)),
//         },
//         GridToChunkIDCase {
//             description: "".to_string(),
//             position: IVec3::new(world_boundary + 1, world_boundary + 1, world_boundary + 1),
//             expected_chunk_id: None,
//         },
//         GridToChunkIDCase {
//             description: "".to_string(),
//             position: IVec3::new(-world_boundary, -world_boundary, -world_boundary),
//             expected_chunk_id: Some(chunk::ID(0)),
//         },
//         GridToChunkIDCase {
//             description: "".to_string(),
//             position: IVec3::new(
//                 -world_boundary - 1,
//                 -world_boundary - 1,
//                 -world_boundary - 1,
//             ),
//             expected_chunk_id: None,
//         },
//     ];

//     for test_case in test_cases {
//         test_case.check(&test_world);
//     }
// }

// struct WorldToChunkIDCase {
//     description: String,
//     world_position: Vec3,
//     expected_chunk_id: Option<chunk::ID>,
// }

// impl WorldToChunkIDCase {
//     pub fn check(&self, world: &World) {
//         let chunk_id = world.grid.world_to_chunk_id(self.world_position);

//         assert_eq!(chunk_id, self.expected_chunk_id, "{:?}", self.description);
//     }
// }

// #[test]
// fn world_to_chunk_id() {
//     let test_world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

//     let world_boundary = test_world.grid.world_boundary as f32;
//     let world_volume = test_world.grid.world_volume;

//     let test_cases = vec![
//         WorldToChunkIDCase {
//             description: "".to_string(),
//             world_position: Vec3::new(0.0, 0.0, 0.0),
//             expected_chunk_id: Some(chunk::ID((world_volume - 1) / 2)),
//         },
//         WorldToChunkIDCase {
//             description: "".to_string(),
//             world_position: Vec3::new(world_boundary, world_boundary, world_boundary),
//             expected_chunk_id: Some(chunk::ID(world_volume - 1)),
//         },
//         WorldToChunkIDCase {
//             description: "".to_string(),
//             world_position: Vec3::new(
//                 world_boundary + 1.0,
//                 world_boundary + 1.0,
//                 world_boundary + 1.0,
//             ),
//             expected_chunk_id: None,
//         },
//         WorldToChunkIDCase {
//             description: "".to_string(),
//             world_position: Vec3::new(-world_boundary, -world_boundary, -world_boundary),
//             expected_chunk_id: Some(chunk::ID(0)),
//         },
//         WorldToChunkIDCase {
//             description: "".to_string(),
//             world_position: Vec3::new(
//                 -world_boundary - 1.0,
//                 -world_boundary - 1.0,
//                 -world_boundary - 1.0,
//             ),
//             expected_chunk_id: None,
//         },
//     ];

//     for test_case in test_cases {
//         test_case.check(&test_world);
//     }
// }

// struct ChunkIDToPositionCase {
//     description: String,
//     chunk_id: chunk::ID,
//     expected_chunk_coordinates: Option<IVec3>,
// }

// impl ChunkIDToPositionCase {
//     pub fn check(&self, world: &World) {
//         let chunk_coordinates = world.grid.chunk_id_to_chunk_coordinates(self.chunk_id);

//         assert_eq!(
//             chunk_coordinates, self.expected_chunk_coordinates,
//             "{:?}",
//             self.description
//         );
//     }
// }

// #[test]
// fn chunk_id_to_chunk_position() {
//     let test_world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

//     let chunk_index_max = test_world.grid.world_volume - 1;
//     let world_radius = test_world.grid.world_radius as i32;

//     let test_cases = vec![
//         ChunkIDToPositionCase {
//             description: "".to_string(),
//             chunk_id: chunk::ID(0),
//             expected_chunk_coordinates: Some(IVec3::new(
//                 -world_radius,
//                 -world_radius,
//                 -world_radius,
//             )),
//         },
//         ChunkIDToPositionCase {
//             description: "".to_string(),
//             chunk_id: chunk::ID(chunk_index_max / 2),
//             expected_chunk_coordinates: Some(IVec3::new(0, 0, 0)),
//         },
//         ChunkIDToPositionCase {
//             description: "".to_string(),
//             chunk_id: chunk::ID(chunk_index_max),
//             expected_chunk_coordinates: Some(IVec3::new(world_radius, world_radius, world_radius)),
//         },
//         ChunkIDToPositionCase {
//             description: "".to_string(),
//             chunk_id: chunk::ID(chunk_index_max + 1),
//             expected_chunk_coordinates: None,
//         },
//     ];

//     for test_case in test_cases {
//         test_case.check(&test_world);
//     }
// }

// struct GridToBlockIDCase {
//     description: String,
//     position: IVec3,
//     expected_block_id: Option<block::ID>,
// }

// impl GridToBlockIDCase {
//     pub fn check(&self, world: &World) {
//         let block_id = world.grid.position_to_block_id(self.position);

//         assert_eq!(block_id, self.expected_block_id, "{:?}", self.description);
//     }
// }

// #[test]
// fn grid_to_block_id() {
//     let test_world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

//     let block_index_max = test_world.grid.chunk_volume - 1;
//     let chunk_radius = test_world.grid.chunk_radius as i32;
//     let chunk_size = test_world.grid.chunk_size as i32;
//     let world_boundary = test_world.grid.world_boundary as i32;

//     let test_cases = vec![
//         GridToBlockIDCase {
//             description: "".to_string(),
//             position: IVec3::new(0, 0, 0),
//             expected_block_id: Some(block::ID(block_index_max / 2)),
//         },
//         GridToBlockIDCase {
//             description: "".to_string(),
//             position: IVec3::new(
//                 -world_boundary - 1,
//                 -world_boundary - 1,
//                 -world_boundary - 1,
//             ),
//             expected_block_id: None,
//         },
//         GridToBlockIDCase {
//             description: "".to_string(),
//             position: IVec3::new(world_boundary + 1, world_boundary + 1, world_boundary + 1),
//             expected_block_id: None,
//         },
//         GridToBlockIDCase {
//             description: "".to_string(),
//             position: IVec3::new(chunk_size, chunk_size, chunk_size),
//             expected_block_id: Some(block::ID(block_index_max / 2)),
//         },
//         GridToBlockIDCase {
//             description: "".to_string(),
//             position: IVec3::new(-chunk_size, -chunk_size, -chunk_size),
//             expected_block_id: Some(block::ID(block_index_max / 2)),
//         },
//         GridToBlockIDCase {
//             description: "".to_string(),
//             position: IVec3::new(chunk_radius, chunk_radius, chunk_radius),
//             expected_block_id: Some(block::ID(block_index_max)),
//         },
//         GridToBlockIDCase {
//             description: "".to_string(),
//             position: IVec3::new(-chunk_radius, -chunk_radius, -chunk_radius),
//             expected_block_id: Some(block::ID(0)),
//         },
//         GridToBlockIDCase {
//             description: "".to_string(),
//             position: IVec3::new(
//                 chunk_size + chunk_radius,
//                 chunk_size + chunk_radius,
//                 chunk_size + chunk_radius,
//             ),
//             expected_block_id: Some(block::ID(block_index_max)),
//         },
//         GridToBlockIDCase {
//             description: "".to_string(),
//             position: IVec3::new(
//                 -chunk_size - chunk_radius,
//                 -chunk_size - chunk_radius,
//                 -chunk_size - chunk_radius,
//             ),
//             expected_block_id: Some(block::ID(0)),
//         },
//     ];

//     for test_case in test_cases {
//         test_case.check(&test_world);
//     }
// }

// struct BlockIDToPositionCase {
//     description: String,
//     block_id: block::ID,
//     expected_block_coordinates: Option<IVec3>,
// }

// impl BlockIDToPositionCase {
//     pub fn check(&self, world: &World) {
//         let block_coordinates = world.grid.block_id_to_block_coordinates(self.block_id);

//         assert_eq!(
//             block_coordinates, self.expected_block_coordinates,
//             "{:?}",
//             self.description
//         );
//     }
// }

// #[test]
// fn block_id_to_block_position() {
//     let test_world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

//     let block_index_max = test_world.grid.chunk_volume - 1;
//     let chunk_radius = test_world.grid.chunk_radius as i32;

//     let test_cases = vec![
//         BlockIDToPositionCase {
//             description: "".to_string(),
//             block_id: block::ID(block_index_max / 2),
//             expected_block_coordinates: Some(IVec3::new(0, 0, 0)),
//         },
//         BlockIDToPositionCase {
//             description: "".to_string(),
//             block_id: block::ID(block_index_max + 1),
//             expected_block_coordinates: None,
//         },
//         BlockIDToPositionCase {
//             description: "".to_string(),
//             block_id: block::ID(0),
//             expected_block_coordinates: Some(IVec3::new(
//                 -chunk_radius,
//                 -chunk_radius,
//                 -chunk_radius,
//             )),
//         },
//         BlockIDToPositionCase {
//             description: "".to_string(),
//             block_id: block::ID(block_index_max),
//             expected_block_coordinates: Some(IVec3::new(chunk_radius, chunk_radius, chunk_radius)),
//         },
//     ];

//     for test_case in test_cases {
//         test_case.check(&test_world);
//     }
// }

// struct GridToIDsCase {
//     description: String,
//     position: IVec3,
//     expected_chunk_id: chunk::ID,
//     expected_block_id: block::ID,
// }

// impl GridToIDsCase {
//     pub fn check(&self, world: &World) {
//         let ids = world.grid.position_to_ids(self.position);

//         assert!(ids.is_some(), "{:?}", self.description);

//         let (chunk_id, block_id) = ids.unwrap();

//         assert_eq!(chunk_id, self.expected_chunk_id, "{:?}", self.description);

//         assert_eq!(block_id, self.expected_block_id, "{:?}", self.description);
//     }
// }

// #[test]
// fn grid_to_ids() {
//     let test_world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

//     let block_index_max = test_world.grid.chunk_volume - 1;
//     let chunk_index_max = test_world.grid.world_volume - 1;

//     let world_radius = test_world.grid.world_radius as i32;
//     let world_size = test_world.grid.world_size as i32;
//     let chunk_radius = test_world.grid.chunk_radius as i32;
//     let chunk_size = test_world.grid.chunk_size as i32;

//     let world_boundary = test_world.grid.world_boundary as i32;

//     let test_cases = vec![
//         GridToIDsCase {
//             description: "".to_string(),
//             position: IVec3::new(0, 0, 0),
//             expected_chunk_id: chunk::ID(chunk_index_max / 2),
//             expected_block_id: block::ID(block_index_max / 2),
//         },
//         GridToIDsCase {
//             description: "".to_string(),
//             position: IVec3::new(world_boundary, world_boundary, world_boundary),
//             expected_chunk_id: chunk::ID(chunk_index_max),
//             expected_block_id: block::ID(block_index_max),
//         },
//         GridToIDsCase {
//             description: "".to_string(),
//             position: IVec3::new(-world_boundary, -world_boundary, -world_boundary),
//             expected_chunk_id: chunk::ID(0),
//             expected_block_id: block::ID(0),
//         },
//         GridToIDsCase {
//             description: "".to_string(),
//             position: IVec3::new(
//                 -chunk_size - chunk_radius,
//                 -chunk_size - chunk_radius,
//                 -chunk_size - chunk_radius,
//             ),
//             expected_chunk_id: {
//                 let chunk_coordinates = IVec3::new(-1, -1, -1);

//                 let chunk_index = (chunk_coordinates.x + world_radius)
//                     + (chunk_coordinates.y + world_radius) * world_size
//                     + (chunk_coordinates.z + world_radius) * world_size * world_size;

//                 chunk::ID(chunk_index as u32)
//             },
//             expected_block_id: block::ID(0),
//         },
//         GridToIDsCase {
//             description: "".to_string(),
//             position: IVec3::new(
//                 chunk_size + chunk_radius,
//                 chunk_size + chunk_radius,
//                 chunk_size + chunk_radius,
//             ),
//             expected_chunk_id: {
//                 let chunk_coordinates = IVec3::new(1, 1, 1);

//                 let chunk_index = (chunk_coordinates.x + world_radius)
//                     + (chunk_coordinates.y + world_radius) * world_size
//                     + (chunk_coordinates.z + world_radius) * world_size * world_size;

//                 chunk::ID(chunk_index as u32)
//             },
//             expected_block_id: block::ID(block_index_max),
//         },
//     ];

//     for test_case in test_cases {
//         test_case.check(&test_world);
//     }
// }

// struct IDsToGridCase {
//     description: String,
//     chunk_id: chunk::ID,
//     block_id: block::ID,
//     expected_position: Option<IVec3>,
// }

// impl IDsToGridCase {
//     pub fn check(&self, world: &World) {
//         let position = world.grid.ids_to_position(self.chunk_id, self.block_id);

//         assert_eq!(position, self.expected_position, "{:?}", self.description);
//     }
// }

// #[test]
// fn ids_to_grid() {
//     let test_world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

//     let block_index_max = test_world.grid.chunk_volume - 1;
//     let chunk_index_max = test_world.grid.world_volume - 1;

//     let world_radius = test_world.grid.world_radius as i32;
//     let world_size = test_world.grid.world_size as i32;
//     let chunk_radius = test_world.grid.chunk_radius as i32;
//     let chunk_size = test_world.grid.chunk_size as i32;

//     let world_boundary = test_world.grid.world_boundary as i32;

//     let test_cases = vec![
//         IDsToGridCase {
//             description: "ids at (0, 0, 0)".to_string(),
//             chunk_id: chunk::ID(chunk_index_max / 2),
//             block_id: block::ID(block_index_max / 2),
//             expected_position: Some(IVec3::new(0, 0, 0)),
//         },
//         IDsToGridCase {
//             description: String::from("ids at (world_boundary, world_boundary, world_boundary)"),
//             chunk_id: chunk::ID(chunk_index_max),
//             block_id: block::ID(block_index_max),
//             expected_position: Some(IVec3::new(world_boundary, world_boundary, world_boundary)),
//         },
//         IDsToGridCase {
//             description: "ids at (-world_boundary, -world_boundary, -world_boundary)".to_string(),
//             chunk_id: chunk::ID(0),
//             block_id: block::ID(0),
//             expected_position: Some(IVec3::new(
//                 -world_boundary,
//                 -world_boundary,
//                 -world_boundary,
//             )),
//         },
//         IDsToGridCase {
//             description: "ids at minimum of chunk (-1, -1, -1)".to_string(),
//             chunk_id: {
//                 let chunk_coordinates = IVec3::new(-1, -1, -1);

//                 let chunk_index = (chunk_coordinates.x + world_radius)
//                     + (chunk_coordinates.y + world_radius) * world_size
//                     + (chunk_coordinates.z + world_radius) * world_size * world_size;

//                 chunk::ID(chunk_index as u32)
//             },
//             block_id: block::ID(0),
//             expected_position: Some(IVec3::new(
//                 -chunk_size - chunk_radius,
//                 -chunk_size - chunk_radius,
//                 -chunk_size - chunk_radius,
//             )),
//         },
//         IDsToGridCase {
//             description: "ids at max of chunk (1, 1, 1)".to_string(),
//             chunk_id: {
//                 let chunk_coordinates = IVec3::new(1, 1, 1);

//                 let chunk_index = (chunk_coordinates.x + world_radius)
//                     + (chunk_coordinates.y + world_radius) * world_size
//                     + (chunk_coordinates.z + world_radius) * world_size * world_size;

//                 chunk::ID(chunk_index as u32)
//             },
//             block_id: block::ID(block_index_max),
//             expected_position: Some(IVec3::new(
//                 chunk_size + chunk_radius,
//                 chunk_size + chunk_radius,
//                 chunk_size + chunk_radius,
//             )),
//         },
//     ];

//     for test_case in test_cases {
//         test_case.check(&test_world);
//     }
// }
