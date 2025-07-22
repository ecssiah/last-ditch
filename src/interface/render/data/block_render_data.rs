use crate::simulation::{self};
use glam::IVec2;
use std::collections::HashMap;

#[derive(Debug)]
pub struct BlockAtlasData {
    pub tile_index_array: [[u32; 2]; 6],
}

impl BlockAtlasData {
    #[rustfmt::skip]
    pub fn setup() -> HashMap<simulation::state::world::block::Kind, BlockAtlasData> {
        use simulation::state::world::block::Kind;

        HashMap::from([
            (
                Kind::Engraved1,
                BlockAtlasData {
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
                BlockAtlasData {
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
                BlockAtlasData {
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
                BlockAtlasData {
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
                BlockAtlasData {
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
                BlockAtlasData {
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
                BlockAtlasData {
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
                BlockAtlasData {
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
                BlockAtlasData {
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
                BlockAtlasData {
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
                BlockAtlasData {
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
                BlockAtlasData {
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
                BlockAtlasData {
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
                BlockAtlasData {
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
                BlockAtlasData {
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
                BlockAtlasData {
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
                BlockAtlasData {
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
                BlockAtlasData {
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
                BlockAtlasData {
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

    pub fn uv_coordinates(
        tile_coordinates: IVec2,
        tile_size: i32,
        tile_atlas_size: IVec2,
    ) -> [[f32; 2]; 4] {
        let block_size = simulation::consts::BLOCK_SIZE as i32;

        let u_min = (tile_coordinates.x * tile_size) as f32 / tile_atlas_size.x as f32;
        let v_min = (tile_coordinates.y * tile_size) as f32 / tile_atlas_size.y as f32;
        let u_max =
            ((tile_coordinates.x + block_size) * tile_size) as f32 / tile_atlas_size.x as f32;
        let v_max =
            ((tile_coordinates.y + block_size) * tile_size) as f32 / tile_atlas_size.y as f32;

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
