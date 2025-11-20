use crate::simulation::{
    constants::CELL_RADIUS,
    state::world::{block, grid},
};
use std::collections::HashMap;
use ultraviolet::{IVec3, UVec2, Vec3};

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
    pub fn setup_tile_coordinates_map() -> HashMap<block::Kind, HashMap<grid::Direction, [u32; 2]>> {
        HashMap::from([
            (
                block::Kind::Engraved1,
                HashMap::from([
                    (grid::Direction::XPYOZO, [0, 0]),
                    (grid::Direction::XNYOZO, [0, 0]),
                    (grid::Direction::XOYPZO, [0, 0]),
                    (grid::Direction::XOYNZO, [0, 0]),
                    (grid::Direction::XOYOZP, [0, 0]),
                    (grid::Direction::XOYOZN, [0, 0]),
                ]),
            ),
            (
                block::Kind::Engraved2,
                HashMap::from([
                    (grid::Direction::XPYOZO, [1, 0]),
                    (grid::Direction::XNYOZO, [1, 0]),
                    (grid::Direction::XOYPZO, [1, 0]),
                    (grid::Direction::XOYNZO, [1, 0]),
                    (grid::Direction::XOYOZP, [1, 0]),
                    (grid::Direction::XOYOZN, [1, 0]),
                ]),
            ),
            (
                block::Kind::Stone1,
                HashMap::from([
                    (grid::Direction::XPYOZO, [0, 1]),
                    (grid::Direction::XNYOZO, [0, 1]),
                    (grid::Direction::XOYPZO, [0, 1]),
                    (grid::Direction::XOYNZO, [0, 1]),
                    (grid::Direction::XOYOZP, [0, 1]),
                    (grid::Direction::XOYOZN, [0, 1]),
                ]),
            ),
            (
                block::Kind::Stone2,
                HashMap::from([
                    (grid::Direction::XPYOZO, [1, 1]),
                    (grid::Direction::XNYOZO, [1, 1]),
                    (grid::Direction::XOYPZO, [1, 1]),
                    (grid::Direction::XOYNZO, [1, 1]),
                    (grid::Direction::XOYOZP, [1, 1]),
                    (grid::Direction::XOYOZN, [1, 1]),
                ]),
            ),
            (
                block::Kind::Polished1,
                HashMap::from([
                    (grid::Direction::XPYOZO, [0, 2]),
                    (grid::Direction::XNYOZO, [0, 2]),
                    (grid::Direction::XOYPZO, [0, 2]),
                    (grid::Direction::XOYNZO, [0, 2]),
                    (grid::Direction::XOYOZP, [0, 2]),
                    (grid::Direction::XOYOZN, [0, 2]),
                ]),
            ),
            (
                block::Kind::Polished2,
                HashMap::from([
                    (grid::Direction::XPYOZO, [1, 2]),
                    (grid::Direction::XNYOZO, [1, 2]),
                    (grid::Direction::XOYPZO, [1, 2]),
                    (grid::Direction::XOYNZO, [1, 2]),
                    (grid::Direction::XOYOZP, [1, 2]),
                    (grid::Direction::XOYOZN, [1, 2]),
                ]),
            ),
            (
                block::Kind::Icon1,
                HashMap::from([
                    (grid::Direction::XPYOZO, [0, 3]),
                    (grid::Direction::XNYOZO, [0, 3]),
                    (grid::Direction::XOYPZO, [0, 3]),
                    (grid::Direction::XOYNZO, [0, 3]),
                    (grid::Direction::XOYOZP, [0, 3]),
                    (grid::Direction::XOYOZN, [0, 3]),
                ]),
            ),
            (
                block::Kind::Icon2,
                HashMap::from([
                    (grid::Direction::XPYOZO, [1, 3]),
                    (grid::Direction::XNYOZO, [1, 3]),
                    (grid::Direction::XOYPZO, [1, 3]),
                    (grid::Direction::XOYNZO, [1, 3]),
                    (grid::Direction::XOYOZP, [1, 3]),
                    (grid::Direction::XOYOZN, [1, 3]),
                ]),
            ),
            (
                block::Kind::Icon3,
                HashMap::from([
                    (grid::Direction::XPYOZO, [2, 3]),
                    (grid::Direction::XNYOZO, [2, 3]),
                    (grid::Direction::XOYPZO, [2, 3]),
                    (grid::Direction::XOYNZO, [2, 3]),
                    (grid::Direction::XOYOZP, [2, 3]),
                    (grid::Direction::XOYOZN, [2, 3]),
                ]),
            ),
            (
                block::Kind::Icon4,
                HashMap::from([
                    (grid::Direction::XPYOZO, [3, 3]),
                    (grid::Direction::XNYOZO, [3, 3]),
                    (grid::Direction::XOYPZO, [3, 3]),
                    (grid::Direction::XOYNZO, [3, 3]),
                    (grid::Direction::XOYOZP, [3, 3]),
                    (grid::Direction::XOYOZN, [3, 3]),
                ]),
            ),
            (
                block::Kind::MagentaStone,
                HashMap::from([
                    (grid::Direction::XPYOZO, [0, 4]),
                    (grid::Direction::XNYOZO, [0, 4]),
                    (grid::Direction::XOYPZO, [0, 4]),
                    (grid::Direction::XOYNZO, [0, 4]),
                    (grid::Direction::XOYOZP, [0, 4]),
                    (grid::Direction::XOYOZN, [0, 4]),
                ]),
            ),
            (
                block::Kind::PurpleStone,
                HashMap::from([
                    (grid::Direction::XPYOZO, [1, 4]),
                    (grid::Direction::XNYOZO, [1, 4]),
                    (grid::Direction::XOYPZO, [1, 4]),
                    (grid::Direction::XOYNZO, [1, 4]),
                    (grid::Direction::XOYOZP, [1, 4]),
                    (grid::Direction::XOYOZN, [1, 4]),
                ]),
            ),
            (
                block::Kind::TealStone,
                HashMap::from([
                    (grid::Direction::XPYOZO, [2, 4]),
                    (grid::Direction::XNYOZO, [2, 4]),
                    (grid::Direction::XOYPZO, [2, 4]),
                    (grid::Direction::XOYNZO, [2, 4]),
                    (grid::Direction::XOYOZP, [2, 4]),
                    (grid::Direction::XOYOZN, [2, 4]),
                ]),
            ),
            (
                block::Kind::CrimsonStone,
                HashMap::from([
                    (grid::Direction::XPYOZO, [3, 4]),
                    (grid::Direction::XNYOZO, [3, 4]),
                    (grid::Direction::XOYPZO, [3, 4]),
                    (grid::Direction::XOYNZO, [3, 4]),
                    (grid::Direction::XOYOZP, [3, 4]),
                    (grid::Direction::XOYOZN, [3, 4]),
                ]),
            ),
            (
                block::Kind::North,
                HashMap::from([
                    (grid::Direction::XPYOZO, [0, 5]),
                    (grid::Direction::XNYOZO, [0, 5]),
                    (grid::Direction::XOYPZO, [0, 5]),
                    (grid::Direction::XOYNZO, [0, 5]),
                    (grid::Direction::XOYOZP, [0, 5]),
                    (grid::Direction::XOYOZN, [0, 5]),
                ]),
            ),
            (
                block::Kind::West,
                HashMap::from([
                    (grid::Direction::XPYOZO, [1, 5]),
                    (grid::Direction::XNYOZO, [1, 5]),
                    (grid::Direction::XOYPZO, [1, 5]),
                    (grid::Direction::XOYNZO, [1, 5]),
                    (grid::Direction::XOYOZP, [1, 5]),
                    (grid::Direction::XOYOZN, [1, 5]),
                ]),
            ),
            (
                block::Kind::South,
                HashMap::from([
                    (grid::Direction::XPYOZO, [2, 5]),
                    (grid::Direction::XNYOZO, [2, 5]),
                    (grid::Direction::XOYPZO, [2, 5]),
                    (grid::Direction::XOYNZO, [2, 5]),
                    (grid::Direction::XOYOZP, [2, 5]),
                    (grid::Direction::XOYOZN, [2, 5]),
                ]),
            ),
            (
                block::Kind::East,
                HashMap::from([
                    (grid::Direction::XPYOZO, [3, 5]),
                    (grid::Direction::XNYOZO, [3, 5]),
                    (grid::Direction::XOYPZO, [3, 5]),
                    (grid::Direction::XOYNZO, [3, 5]),
                    (grid::Direction::XOYOZP, [3, 5]),
                    (grid::Direction::XOYOZN, [3, 5]),
                ]),
            ),
            (
                block::Kind::EsayaBlock,
                HashMap::from([
                    (grid::Direction::XPYOZO, [0, 6]),
                    (grid::Direction::XNYOZO, [0, 6]),
                    (grid::Direction::XOYPZO, [0, 6]),
                    (grid::Direction::XOYNZO, [0, 6]),
                    (grid::Direction::XOYOZP, [0, 6]),
                    (grid::Direction::XOYOZN, [0, 6]),
                ]),
            ),
        ])
    }

    #[rustfmt::skip]
    pub fn face_vertex_position_array(position: IVec3, direction: grid::Direction) -> [[f32; 3]; 4] {
        let position = Vec3::from(position);

        match direction {
            grid::Direction::XPYOZO => [
                [position.x + CELL_RADIUS, position.y - CELL_RADIUS, position.z - CELL_RADIUS],
                [position.x + CELL_RADIUS, position.y + CELL_RADIUS, position.z - CELL_RADIUS],
                [position.x + CELL_RADIUS, position.y + CELL_RADIUS, position.z + CELL_RADIUS],
                [position.x + CELL_RADIUS, position.y - CELL_RADIUS, position.z + CELL_RADIUS],
            ],
            grid::Direction::XNYOZO => [
                [position.x - CELL_RADIUS, position.y + CELL_RADIUS, position.z - CELL_RADIUS],
                [position.x - CELL_RADIUS, position.y - CELL_RADIUS, position.z - CELL_RADIUS],
                [position.x - CELL_RADIUS, position.y - CELL_RADIUS, position.z + CELL_RADIUS],
                [position.x - CELL_RADIUS, position.y + CELL_RADIUS, position.z + CELL_RADIUS],
            ],
            grid::Direction::XOYPZO => [
                [position.x + CELL_RADIUS, position.y + CELL_RADIUS, position.z - CELL_RADIUS],
                [position.x - CELL_RADIUS, position.y + CELL_RADIUS, position.z - CELL_RADIUS],
                [position.x - CELL_RADIUS, position.y + CELL_RADIUS, position.z + CELL_RADIUS],
                [position.x + CELL_RADIUS, position.y + CELL_RADIUS, position.z + CELL_RADIUS],
            ],
            grid::Direction::XOYNZO => [
                [position.x - CELL_RADIUS, position.y - CELL_RADIUS, position.z - CELL_RADIUS],
                [position.x + CELL_RADIUS, position.y - CELL_RADIUS, position.z - CELL_RADIUS],
                [position.x + CELL_RADIUS, position.y - CELL_RADIUS, position.z + CELL_RADIUS],
                [position.x - CELL_RADIUS, position.y - CELL_RADIUS, position.z + CELL_RADIUS],
            ],
            grid::Direction::XOYOZP => [
                [position.x - CELL_RADIUS, position.y - CELL_RADIUS, position.z + CELL_RADIUS],
                [position.x + CELL_RADIUS, position.y - CELL_RADIUS, position.z + CELL_RADIUS],
                [position.x + CELL_RADIUS, position.y + CELL_RADIUS, position.z + CELL_RADIUS],
                [position.x - CELL_RADIUS, position.y + CELL_RADIUS, position.z + CELL_RADIUS],
            ],
            grid::Direction::XOYOZN => [
                [position.x - CELL_RADIUS, position.y + CELL_RADIUS, position.z - CELL_RADIUS],
                [position.x + CELL_RADIUS, position.y + CELL_RADIUS, position.z - CELL_RADIUS],
                [position.x + CELL_RADIUS, position.y - CELL_RADIUS, position.z - CELL_RADIUS],
                [position.x - CELL_RADIUS, position.y - CELL_RADIUS, position.z - CELL_RADIUS],
            ],
            _ => panic!("Invalid face direction"),
        }
    }

    pub fn tile_uv_array(coordinates: &[u32; 2], size: u32, atlas_size: UVec2) -> [[f32; 2]; 4] {
        let u_min = (coordinates[0] * size) as f32 / atlas_size.x as f32;
        let v_min = (coordinates[1] * size) as f32 / atlas_size.y as f32;
        let u_max = ((coordinates[0] + 1) * size) as f32 / atlas_size.x as f32;
        let v_max = ((coordinates[1] + 1) * size) as f32 / atlas_size.y as f32;

        [
            [u_min, v_max],
            [u_max, v_max],
            [u_max, v_min],
            [u_min, v_min],
        ]
    }
}
