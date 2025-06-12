use crate::simulation::{self};
use std::collections::HashMap;

#[derive(Debug)]
pub struct BlockRenderData {
    pub tile_index_array: [[u32; 2]; 6],
}

impl BlockRenderData {
    #[rustfmt::skip]
    pub fn setup() -> HashMap<simulation::world::block::Kind, BlockRenderData> {
        use simulation::world::block::Kind;

        HashMap::from([
            (
                Kind::Engraved1,
                BlockRenderData {
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
                BlockRenderData {
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
                BlockRenderData {
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
                BlockRenderData {
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
                BlockRenderData {
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
                BlockRenderData {
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
                BlockRenderData {
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
                BlockRenderData {
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
                BlockRenderData {
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
                BlockRenderData {
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
                BlockRenderData {
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
                BlockRenderData {
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
                BlockRenderData {
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
                BlockRenderData {
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
                BlockRenderData {
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
                BlockRenderData {
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
                BlockRenderData {
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
                BlockRenderData {
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
        ])
    }

    pub fn direction_to_index(&self, direction: simulation::world::grid::Direction) -> usize {
        use simulation::world::grid::Direction;

        match direction {
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
