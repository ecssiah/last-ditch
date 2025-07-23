use crate::simulation::{self, consts::BLOCK_RADIUS, state::world::grid};
use glam::{IVec2, IVec3};
use std::collections::HashMap;

#[derive(Debug)]
pub struct BlockRenderInfo {
    pub tile_index_array: [[u32; 2]; 6],
}

impl BlockRenderInfo {
    #[rustfmt::skip]
    pub fn setup() -> HashMap<simulation::state::world::block::Kind, BlockRenderInfo> {
        use simulation::state::world::block::Kind;

        HashMap::from([
            (
                Kind::Engraved1,
                BlockRenderInfo {
                    tile_index_array: [
                        [0, 0],
                        [0, 0],
                        [0, 0],
                        [0, 0],
                        [0, 0],
                        [0, 0],
                    ],
                },
            ),
            (
                Kind::Engraved2,
                BlockRenderInfo {
                    tile_index_array: [
                        [1, 0],
                        [1, 0],
                        [1, 0],
                        [1, 0],
                        [1, 0],
                        [1, 0],
                    ],
                },
            ),
            (
                Kind::Stone1,
                BlockRenderInfo {
                    tile_index_array: [
                        [0, 1],
                        [0, 1],
                        [0, 1],
                        [0, 1],
                        [0, 1],
                        [0, 1],
                    ],
                },
            ),
            (
                Kind::Stone2,
                BlockRenderInfo {
                    tile_index_array: [
                        [1, 1],
                        [1, 1],
                        [1, 1],
                        [1, 1],
                        [1, 1],
                        [1, 1],
                    ],
                },
            ),
            (
                Kind::Polished1,
                BlockRenderInfo {
                    tile_index_array: [
                        [0, 2],
                        [0, 2],
                        [0, 2],
                        [0, 2],
                        [0, 2],
                        [0, 2],
                    ],
                },
            ),
            (
                Kind::Polished2,
                BlockRenderInfo {
                    tile_index_array: [
                        [1, 2],
                        [1, 2],
                        [1, 2],
                        [1, 2],
                        [1, 2],
                        [1, 2],
                    ],
                },
            ),
            (
                Kind::Icon1,
                BlockRenderInfo {
                    tile_index_array: [
                        [0, 3],
                        [0, 3],
                        [0, 3],
                        [0, 3],
                        [0, 3],
                        [0, 3],
                    ],
                },
            ),
            (
                Kind::Icon2,
                BlockRenderInfo {
                    tile_index_array: [
                        [1, 3],
                        [1, 3],
                        [1, 3],
                        [1, 3],
                        [1, 3],
                        [1, 3],
                    ],
                },
            ),
            (
                Kind::Icon3,
                BlockRenderInfo {
                    tile_index_array: [
                        [2, 3],
                        [2, 3],
                        [2, 3],
                        [2, 3],
                        [2, 3],
                        [2, 3],
                    ],
                },
            ),
            (
                Kind::Icon4,
                BlockRenderInfo {
                    tile_index_array: [
                        [3, 3],
                        [3, 3],
                        [3, 3],
                        [3, 3],
                        [3, 3],
                        [3, 3],
                    ],
                },
            ),
            (
                Kind::MagentaStone,
                BlockRenderInfo {
                    tile_index_array: [
                        [0, 4],
                        [0, 4],
                        [0, 4],
                        [0, 4],
                        [0, 4],
                        [0, 4],
                    ],
                },
            ),
            (
                Kind::PurpleStone,
                BlockRenderInfo {
                    tile_index_array: [
                        [1, 4],
                        [1, 4],
                        [1, 4],
                        [1, 4],
                        [1, 4],
                        [1, 4],
                    ],
                },
            ),
            (
                Kind::TealStone,
                BlockRenderInfo {
                    tile_index_array: [
                        [2, 4],
                        [2, 4],
                        [2, 4],
                        [2, 4],
                        [2, 4],
                        [2, 4],
                    ],
                },
            ),
            (
                Kind::CrimsonStone,
                BlockRenderInfo {
                    tile_index_array: [
                        [3, 4],
                        [3, 4],
                        [3, 4],
                        [3, 4],
                        [3, 4],
                        [3, 4],
                    ],
                },
            ),
            (
                Kind::North,
                BlockRenderInfo {
                    tile_index_array: [
                        [0, 5],
                        [0, 5],
                        [0, 5],
                        [0, 5],
                        [0, 5],
                        [0, 5],
                    ],
                },
            ),
            (
                Kind::West,
                BlockRenderInfo {
                    tile_index_array: [
                        [1, 5],
                        [1, 5],
                        [1, 5],
                        [1, 5],
                        [1, 5],
                        [1, 5],
                    ],
                },
            ),
            (
                Kind::South,
                BlockRenderInfo {
                    tile_index_array: [
                        [2, 5],
                        [2, 5],
                        [2, 5],
                        [2, 5],
                        [2, 5],
                        [2, 5],
                    ],
                },
            ),
            (
                Kind::East,
                BlockRenderInfo {
                    tile_index_array: [
                        [3, 5],
                        [3, 5],
                        [3, 5],
                        [3, 5],
                        [3, 5],
                        [3, 5],
                    ],
                },
            ),
            (
                Kind::EsayaBlock,
                BlockRenderInfo {
                    tile_index_array: [
                        [0, 6],
                        [0, 6],
                        [0, 6],
                        [0, 6],
                        [0, 6],
                        [0, 6],
                    ],
                },
            ),
        ])
    }

    #[rustfmt::skip]
    pub fn face_vertices(position: IVec3, direction: grid::Direction) -> [[f32; 3]; 4] {
        let position = position.as_vec3();

        match direction {
            grid::Direction::XpYoZo => [
                [position.x + BLOCK_RADIUS, position.y + BLOCK_RADIUS, position.z + BLOCK_RADIUS],
                [position.x + BLOCK_RADIUS, position.y - BLOCK_RADIUS, position.z + BLOCK_RADIUS],
                [position.x + BLOCK_RADIUS, position.y - BLOCK_RADIUS, position.z - BLOCK_RADIUS],
                [position.x + BLOCK_RADIUS, position.y + BLOCK_RADIUS, position.z - BLOCK_RADIUS],
            ],
            grid::Direction::XnYoZo => [
                [position.x - BLOCK_RADIUS, position.y + BLOCK_RADIUS, position.z - BLOCK_RADIUS],
                [position.x - BLOCK_RADIUS, position.y - BLOCK_RADIUS, position.z - BLOCK_RADIUS],
                [position.x - BLOCK_RADIUS, position.y - BLOCK_RADIUS, position.z + BLOCK_RADIUS],
                [position.x - BLOCK_RADIUS, position.y + BLOCK_RADIUS, position.z + BLOCK_RADIUS],
            ],
            grid::Direction::XoYpZo => [
                [position.x + BLOCK_RADIUS, position.y + BLOCK_RADIUS, position.z + BLOCK_RADIUS],
                [position.x + BLOCK_RADIUS, position.y + BLOCK_RADIUS, position.z - BLOCK_RADIUS],
                [position.x - BLOCK_RADIUS, position.y + BLOCK_RADIUS, position.z - BLOCK_RADIUS],
                [position.x - BLOCK_RADIUS, position.y + BLOCK_RADIUS, position.z + BLOCK_RADIUS],
            ],
            grid::Direction::XoYnZo => [
                [position.x + BLOCK_RADIUS, position.y - BLOCK_RADIUS, position.z - BLOCK_RADIUS],
                [position.x + BLOCK_RADIUS, position.y - BLOCK_RADIUS, position.z + BLOCK_RADIUS],
                [position.x - BLOCK_RADIUS, position.y - BLOCK_RADIUS, position.z + BLOCK_RADIUS],
                [position.x - BLOCK_RADIUS, position.y - BLOCK_RADIUS, position.z - BLOCK_RADIUS],
            ],
            grid::Direction::XoYoZp => [
                [position.x - BLOCK_RADIUS, position.y + BLOCK_RADIUS, position.z + BLOCK_RADIUS],
                [position.x - BLOCK_RADIUS, position.y - BLOCK_RADIUS, position.z + BLOCK_RADIUS],
                [position.x + BLOCK_RADIUS, position.y - BLOCK_RADIUS, position.z + BLOCK_RADIUS],
                [position.x + BLOCK_RADIUS, position.y + BLOCK_RADIUS, position.z + BLOCK_RADIUS],
            ],
            grid::Direction::XoYoZn => [
                [position.x + BLOCK_RADIUS, position.y + BLOCK_RADIUS, position.z - BLOCK_RADIUS],
                [position.x + BLOCK_RADIUS, position.y - BLOCK_RADIUS, position.z - BLOCK_RADIUS],
                [position.x - BLOCK_RADIUS, position.y - BLOCK_RADIUS, position.z - BLOCK_RADIUS],
                [position.x - BLOCK_RADIUS, position.y + BLOCK_RADIUS, position.z - BLOCK_RADIUS],
            ],
            _ => panic!("Invalid face direction"),
        }
    }

    pub fn tile_uv_coordinates(coordinates: IVec2, size: i32, atlas_size: IVec2) -> [[f32; 2]; 4] {
        let block_size = simulation::consts::BLOCK_SIZE as i32;

        let u_min = (coordinates.x * size) as f32 / atlas_size.x as f32;
        let v_min = (coordinates.y * size) as f32 / atlas_size.y as f32;
        let u_max = ((coordinates.x + block_size) * size) as f32 / atlas_size.x as f32;
        let v_max = ((coordinates.y + block_size) * size) as f32 / atlas_size.y as f32;

        [
            [u_max, v_max],
            [u_min, v_max],
            [u_min, v_min],
            [u_max, v_min],
        ]
    }

    pub fn face_direction_to_index(
        face_direction: simulation::state::world::grid::Direction,
    ) -> usize {
        use simulation::state::world::grid::Direction;

        match face_direction {
            Direction::XpYoZo => 0,
            Direction::XnYoZo => 1,
            Direction::XoYpZo => 2,
            Direction::XoYnZo => 3,
            Direction::XoYoZp => 4,
            Direction::XoYoZn => 5,
            _ => panic!("invalid face direction"),
        }
    }
}
