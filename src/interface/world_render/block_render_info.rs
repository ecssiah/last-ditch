use crate::simulation::{
    constants::CELL_RADIUS,
    state::world::{block, grid},
};
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
    pub fn get_tile_coordinates(block_kind: block::Kind) -> [[u32; 2]; 6] {
        match block_kind {
            block::Kind::None => { 
                panic!("None block has no tile data!")
            }
            block::Kind::Engraved1 => {
                [[0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0]]
            }
            block::Kind::Engraved2 => {
                [[1, 0], [1, 0], [1, 0], [1, 0], [1, 0], [1, 0]]
            }
            block::Kind::Stone1 => {
                [[0, 1], [0, 1], [0, 1], [0, 1], [0, 1], [0, 1]]
            }
            block::Kind::Stone2 => {
                [[1, 1], [1, 1], [1, 1], [1, 1], [1, 1], [1, 1]]
            }
            block::Kind::Polished1 => {
                [[0, 2], [0, 2], [0, 2], [0, 2], [0, 2], [0, 2]]
            }
            block::Kind::Polished2 => {
                [[1, 2], [1, 2], [1, 2], [1, 2], [1, 2], [1, 2]]
            }
            block::Kind::MagentaStone => {
                [[0, 4], [0, 4], [0, 4], [0, 4], [0, 4], [0, 4]]
            }
            block::Kind::PurpleStone => {
                [[1, 4], [1, 4], [1, 4], [1, 4], [1, 4], [1, 4]]
            }
            block::Kind::TealStone => {
                [[2, 4], [2, 4], [2, 4], [2, 4], [2, 4], [2, 4]]
            }
            block::Kind::CrimsonStone => {
                [[3, 4], [3, 4], [3, 4], [3, 4], [3, 4], [3, 4]]
            }
            block::Kind::Icon1 => {
                [[0, 3], [0, 3], [0, 3], [0, 3], [0, 3], [0, 3]]
            }
            block::Kind::Icon2 => {
                [[1, 3], [1, 3], [1, 3], [1, 3], [1, 3], [1, 3]]
            }
            block::Kind::Icon3 => {
                [[2, 3], [2, 3], [2, 3], [2, 3], [2, 3], [2, 3]]
            }
            block::Kind::Icon4 => {
                [[3, 3], [3, 3], [3, 3], [3, 3], [3, 3], [3, 3]]
            }
            block::Kind::NorthBlock => {
                [[0, 5], [0, 5], [0, 5], [0, 5], [0, 5], [0, 5]]
            }
            block::Kind::WestBlock => {
                [[1, 5], [1, 5], [1, 5], [1, 5], [1, 5], [1, 5]]
            }
            block::Kind::SouthBlock => {
                [[2, 5], [2, 5], [2, 5], [2, 5], [2, 5], [2, 5]]
            }
            block::Kind::EastBlock => {
                [[3, 5], [3, 5], [3, 5], [3, 5], [3, 5], [3, 5]]
            }
            block::Kind::EsayaBlock => {
                [[0, 6], [0, 6], [0, 6], [0, 6], [0, 6], [0, 6]]
            }
        }
    }

    pub fn tile_to_layer(coords: [u32; 2], block_render_info: &BlockRenderInfo) -> u32 {
        let tiles_per_row = block_render_info.tile_atlas_size.x / block_render_info.tile_size;

        coords[1] * tiles_per_row + coords[0]
    }

    #[rustfmt::skip]
    pub fn get_face_vertex_position_array(position: IVec3, direction: grid::Direction) -> [[f32; 3]; 4] {
        let position = Vec3::from(position);

        match direction {
            grid::Direction::East => [
                [position.x + CELL_RADIUS, position.y - CELL_RADIUS, position.z - CELL_RADIUS],
                [position.x + CELL_RADIUS, position.y + CELL_RADIUS, position.z - CELL_RADIUS],
                [position.x + CELL_RADIUS, position.y + CELL_RADIUS, position.z + CELL_RADIUS],
                [position.x + CELL_RADIUS, position.y - CELL_RADIUS, position.z + CELL_RADIUS],
            ],
            grid::Direction::West => [
                [position.x - CELL_RADIUS, position.y + CELL_RADIUS, position.z - CELL_RADIUS],
                [position.x - CELL_RADIUS, position.y - CELL_RADIUS, position.z - CELL_RADIUS],
                [position.x - CELL_RADIUS, position.y - CELL_RADIUS, position.z + CELL_RADIUS],
                [position.x - CELL_RADIUS, position.y + CELL_RADIUS, position.z + CELL_RADIUS],
            ],
            grid::Direction::North => [
                [position.x + CELL_RADIUS, position.y + CELL_RADIUS, position.z - CELL_RADIUS],
                [position.x - CELL_RADIUS, position.y + CELL_RADIUS, position.z - CELL_RADIUS],
                [position.x - CELL_RADIUS, position.y + CELL_RADIUS, position.z + CELL_RADIUS],
                [position.x + CELL_RADIUS, position.y + CELL_RADIUS, position.z + CELL_RADIUS],
            ],
            grid::Direction::South => [
                [position.x - CELL_RADIUS, position.y - CELL_RADIUS, position.z - CELL_RADIUS],
                [position.x + CELL_RADIUS, position.y - CELL_RADIUS, position.z - CELL_RADIUS],
                [position.x + CELL_RADIUS, position.y - CELL_RADIUS, position.z + CELL_RADIUS],
                [position.x - CELL_RADIUS, position.y - CELL_RADIUS, position.z + CELL_RADIUS],
            ],
            grid::Direction::Up => [
                [position.x - CELL_RADIUS, position.y - CELL_RADIUS, position.z + CELL_RADIUS],
                [position.x + CELL_RADIUS, position.y - CELL_RADIUS, position.z + CELL_RADIUS],
                [position.x + CELL_RADIUS, position.y + CELL_RADIUS, position.z + CELL_RADIUS],
                [position.x - CELL_RADIUS, position.y + CELL_RADIUS, position.z + CELL_RADIUS],
            ],
            grid::Direction::Down => [
                [position.x - CELL_RADIUS, position.y + CELL_RADIUS, position.z - CELL_RADIUS],
                [position.x + CELL_RADIUS, position.y + CELL_RADIUS, position.z - CELL_RADIUS],
                [position.x + CELL_RADIUS, position.y - CELL_RADIUS, position.z - CELL_RADIUS],
                [position.x - CELL_RADIUS, position.y - CELL_RADIUS, position.z - CELL_RADIUS],
            ],
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
