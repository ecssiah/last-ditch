use crate::simulation::{self, consts::BLOCK_EXTENT, state::world::grid};
use glam::{IVec3, UVec2};
use std::collections::HashMap;

#[derive(Debug)]
pub struct BlockRenderInfo {
    pub tile_size: u32,
    pub tile_atlas_size: UVec2,
}

impl BlockRenderInfo {
    pub fn new(tile_size: u32, tile_atlas_width: u32, tile_atlas_height: u32) -> Self {
        Self {
            tile_size,
            tile_atlas_size: UVec2::new(tile_atlas_width, tile_atlas_height),
        }
    }

    #[rustfmt::skip]
    pub fn setup_tile_coordinates_map() -> HashMap<simulation::state::world::block::Kind, HashMap<grid::Direction, [u32; 2]>> {
        use simulation::state::world::block::Kind;

        HashMap::from([
            (
                Kind::Engraved1,
                HashMap::from([
                    (grid::Direction::XpYoZo, [0, 0]),
                    (grid::Direction::XnYoZo, [0, 0]),
                    (grid::Direction::XoYpZo, [0, 0]),
                    (grid::Direction::XoYnZo, [0, 0]),
                    (grid::Direction::XoYoZp, [0, 0]),
                    (grid::Direction::XoYoZn, [0, 0]),
                ]),
            ),
            (
                Kind::Engraved2,
                HashMap::from([
                    (grid::Direction::XpYoZo, [1, 0]),
                    (grid::Direction::XnYoZo, [1, 0]),
                    (grid::Direction::XoYpZo, [1, 0]),
                    (grid::Direction::XoYnZo, [1, 0]),
                    (grid::Direction::XoYoZp, [1, 0]),
                    (grid::Direction::XoYoZn, [1, 0]),
                ]),
            ),
            (
                Kind::Stone1,
                HashMap::from([
                    (grid::Direction::XpYoZo, [0, 1]),
                    (grid::Direction::XnYoZo, [0, 1]),
                    (grid::Direction::XoYpZo, [0, 1]),
                    (grid::Direction::XoYnZo, [0, 1]),
                    (grid::Direction::XoYoZp, [0, 1]),
                    (grid::Direction::XoYoZn, [0, 1]),
                ]),
            ),
            (
                Kind::Stone2,
                HashMap::from([
                    (grid::Direction::XpYoZo, [1, 1]),
                    (grid::Direction::XnYoZo, [1, 1]),
                    (grid::Direction::XoYpZo, [1, 1]),
                    (grid::Direction::XoYnZo, [1, 1]),
                    (grid::Direction::XoYoZp, [1, 1]),
                    (grid::Direction::XoYoZn, [1, 1]),
                ]),
            ),
            (
                Kind::Polished1,
                HashMap::from([
                    (grid::Direction::XpYoZo, [0, 2]),
                    (grid::Direction::XnYoZo, [0, 2]),
                    (grid::Direction::XoYpZo, [0, 2]),
                    (grid::Direction::XoYnZo, [0, 2]),
                    (grid::Direction::XoYoZp, [0, 2]),
                    (grid::Direction::XoYoZn, [0, 2]),
                ]),
            ),
            (
                Kind::Polished2,
                HashMap::from([
                    (grid::Direction::XpYoZo, [1, 2]),
                    (grid::Direction::XnYoZo, [1, 2]),
                    (grid::Direction::XoYpZo, [1, 2]),
                    (grid::Direction::XoYnZo, [1, 2]),
                    (grid::Direction::XoYoZp, [1, 2]),
                    (grid::Direction::XoYoZn, [1, 2]),
                ]),
            ),
            (
                Kind::Icon1,
                HashMap::from([
                    (grid::Direction::XpYoZo, [0, 3]),
                    (grid::Direction::XnYoZo, [0, 3]),
                    (grid::Direction::XoYpZo, [0, 3]),
                    (grid::Direction::XoYnZo, [0, 3]),
                    (grid::Direction::XoYoZp, [0, 3]),
                    (grid::Direction::XoYoZn, [0, 3]),
                ]),
            ),
            (
                Kind::Icon2,
                HashMap::from([
                    (grid::Direction::XpYoZo, [1, 3]),
                    (grid::Direction::XnYoZo, [1, 3]),
                    (grid::Direction::XoYpZo, [1, 3]),
                    (grid::Direction::XoYnZo, [1, 3]),
                    (grid::Direction::XoYoZp, [1, 3]),
                    (grid::Direction::XoYoZn, [1, 3]),
                ]),
            ),
            (
                Kind::Icon3,
                HashMap::from([
                    (grid::Direction::XpYoZo, [2, 3]),
                    (grid::Direction::XnYoZo, [2, 3]),
                    (grid::Direction::XoYpZo, [2, 3]),
                    (grid::Direction::XoYnZo, [2, 3]),
                    (grid::Direction::XoYoZp, [2, 3]),
                    (grid::Direction::XoYoZn, [2, 3]),
                ]),
            ),
            (
                Kind::Icon4,
                HashMap::from([
                    (grid::Direction::XpYoZo, [3, 3]),
                    (grid::Direction::XnYoZo, [3, 3]),
                    (grid::Direction::XoYpZo, [3, 3]),
                    (grid::Direction::XoYnZo, [3, 3]),
                    (grid::Direction::XoYoZp, [3, 3]),
                    (grid::Direction::XoYoZn, [3, 3]),
                ]),
            ),
            (
                Kind::MagentaStone,
                HashMap::from([
                    (grid::Direction::XpYoZo, [0, 4]),
                    (grid::Direction::XnYoZo, [0, 4]),
                    (grid::Direction::XoYpZo, [0, 4]),
                    (grid::Direction::XoYnZo, [0, 4]),
                    (grid::Direction::XoYoZp, [0, 4]),
                    (grid::Direction::XoYoZn, [0, 4]),
                ]),
            ),
            (
                Kind::PurpleStone,
                HashMap::from([
                    (grid::Direction::XpYoZo, [1, 4]),
                    (grid::Direction::XnYoZo, [1, 4]),
                    (grid::Direction::XoYpZo, [1, 4]),
                    (grid::Direction::XoYnZo, [1, 4]),
                    (grid::Direction::XoYoZp, [1, 4]),
                    (grid::Direction::XoYoZn, [1, 4]),
                ]),
            ),
            (
                Kind::TealStone,
                HashMap::from([
                    (grid::Direction::XpYoZo, [2, 4]),
                    (grid::Direction::XnYoZo, [2, 4]),
                    (grid::Direction::XoYpZo, [2, 4]),
                    (grid::Direction::XoYnZo, [2, 4]),
                    (grid::Direction::XoYoZp, [2, 4]),
                    (grid::Direction::XoYoZn, [2, 4]),
                ]),
            ),
            (
                Kind::CrimsonStone,
                HashMap::from([
                    (grid::Direction::XpYoZo, [3, 4]),
                    (grid::Direction::XnYoZo, [3, 4]),
                    (grid::Direction::XoYpZo, [3, 4]),
                    (grid::Direction::XoYnZo, [3, 4]),
                    (grid::Direction::XoYoZp, [3, 4]),
                    (grid::Direction::XoYoZn, [3, 4]),
                ]),
            ),
            (
                Kind::North,
                HashMap::from([
                    (grid::Direction::XpYoZo, [0, 5]),
                    (grid::Direction::XnYoZo, [0, 5]),
                    (grid::Direction::XoYpZo, [0, 5]),
                    (grid::Direction::XoYnZo, [0, 5]),
                    (grid::Direction::XoYoZp, [0, 5]),
                    (grid::Direction::XoYoZn, [0, 5]),
                ]),
            ),
            (
                Kind::West,
                HashMap::from([
                    (grid::Direction::XpYoZo, [1, 5]),
                    (grid::Direction::XnYoZo, [1, 5]),
                    (grid::Direction::XoYpZo, [1, 5]),
                    (grid::Direction::XoYnZo, [1, 5]),
                    (grid::Direction::XoYoZp, [1, 5]),
                    (grid::Direction::XoYoZn, [1, 5]),
                ]),
            ),
            (
                Kind::South,
                HashMap::from([
                    (grid::Direction::XpYoZo, [2, 5]),
                    (grid::Direction::XnYoZo, [2, 5]),
                    (grid::Direction::XoYpZo, [2, 5]),
                    (grid::Direction::XoYnZo, [2, 5]),
                    (grid::Direction::XoYoZp, [2, 5]),
                    (grid::Direction::XoYoZn, [2, 5]),
                ]),
            ),
            (
                Kind::East,
                HashMap::from([
                    (grid::Direction::XpYoZo, [3, 5]),
                    (grid::Direction::XnYoZo, [3, 5]),
                    (grid::Direction::XoYpZo, [3, 5]),
                    (grid::Direction::XoYnZo, [3, 5]),
                    (grid::Direction::XoYoZp, [3, 5]),
                    (grid::Direction::XoYoZn, [3, 5]),
                ]),
            ),
            (
                Kind::EsayaBlock,
                HashMap::from([
                    (grid::Direction::XpYoZo, [0, 6]),
                    (grid::Direction::XnYoZo, [0, 6]),
                    (grid::Direction::XoYpZo, [0, 6]),
                    (grid::Direction::XoYnZo, [0, 6]),
                    (grid::Direction::XoYoZp, [0, 6]),
                    (grid::Direction::XoYoZn, [0, 6]),
                ]),
            ),
        ])
    }

    #[rustfmt::skip]
    pub fn face_vertex_position_array(position: IVec3, direction: grid::Direction) -> [[f32; 3]; 4] {
        let position = position.as_vec3();

        match direction {
            grid::Direction::XpYoZo => [
                [position.x + BLOCK_EXTENT, position.y + BLOCK_EXTENT, position.z + BLOCK_EXTENT],
                [position.x + BLOCK_EXTENT, position.y - BLOCK_EXTENT, position.z + BLOCK_EXTENT],
                [position.x + BLOCK_EXTENT, position.y - BLOCK_EXTENT, position.z - BLOCK_EXTENT],
                [position.x + BLOCK_EXTENT, position.y + BLOCK_EXTENT, position.z - BLOCK_EXTENT],
            ],
            grid::Direction::XnYoZo => [
                [position.x - BLOCK_EXTENT, position.y + BLOCK_EXTENT, position.z - BLOCK_EXTENT],
                [position.x - BLOCK_EXTENT, position.y - BLOCK_EXTENT, position.z - BLOCK_EXTENT],
                [position.x - BLOCK_EXTENT, position.y - BLOCK_EXTENT, position.z + BLOCK_EXTENT],
                [position.x - BLOCK_EXTENT, position.y + BLOCK_EXTENT, position.z + BLOCK_EXTENT],
            ],
            grid::Direction::XoYpZo => [
                [position.x + BLOCK_EXTENT, position.y + BLOCK_EXTENT, position.z + BLOCK_EXTENT],
                [position.x + BLOCK_EXTENT, position.y + BLOCK_EXTENT, position.z - BLOCK_EXTENT],
                [position.x - BLOCK_EXTENT, position.y + BLOCK_EXTENT, position.z - BLOCK_EXTENT],
                [position.x - BLOCK_EXTENT, position.y + BLOCK_EXTENT, position.z + BLOCK_EXTENT],
            ],
            grid::Direction::XoYnZo => [
                [position.x + BLOCK_EXTENT, position.y - BLOCK_EXTENT, position.z - BLOCK_EXTENT],
                [position.x + BLOCK_EXTENT, position.y - BLOCK_EXTENT, position.z + BLOCK_EXTENT],
                [position.x - BLOCK_EXTENT, position.y - BLOCK_EXTENT, position.z + BLOCK_EXTENT],
                [position.x - BLOCK_EXTENT, position.y - BLOCK_EXTENT, position.z - BLOCK_EXTENT],
            ],
            grid::Direction::XoYoZp => [
                [position.x - BLOCK_EXTENT, position.y + BLOCK_EXTENT, position.z + BLOCK_EXTENT],
                [position.x - BLOCK_EXTENT, position.y - BLOCK_EXTENT, position.z + BLOCK_EXTENT],
                [position.x + BLOCK_EXTENT, position.y - BLOCK_EXTENT, position.z + BLOCK_EXTENT],
                [position.x + BLOCK_EXTENT, position.y + BLOCK_EXTENT, position.z + BLOCK_EXTENT],
            ],
            grid::Direction::XoYoZn => [
                [position.x + BLOCK_EXTENT, position.y + BLOCK_EXTENT, position.z - BLOCK_EXTENT],
                [position.x + BLOCK_EXTENT, position.y - BLOCK_EXTENT, position.z - BLOCK_EXTENT],
                [position.x - BLOCK_EXTENT, position.y - BLOCK_EXTENT, position.z - BLOCK_EXTENT],
                [position.x - BLOCK_EXTENT, position.y + BLOCK_EXTENT, position.z - BLOCK_EXTENT],
            ],
            _ => panic!("Invalid face direction"),
        }
    }

    pub fn tile_uv_array(
        tile_coordinates: &[u32; 2],
        tile_size: u32,
        tile_atlas_size: UVec2,
    ) -> [[f32; 2]; 4] {
        let u_min = (tile_coordinates[0] * tile_size) as f32 / tile_atlas_size.x as f32;
        let v_min = (tile_coordinates[1] * tile_size) as f32 / tile_atlas_size.y as f32;
        let u_max = ((tile_coordinates[0] + 1) * tile_size) as f32 / tile_atlas_size.x as f32;
        let v_max = ((tile_coordinates[1] + 1) * tile_size) as f32 / tile_atlas_size.y as f32;

        [
            [u_max, v_max],
            [u_min, v_max],
            [u_min, v_min],
            [u_max, v_min],
        ]
    }
}
