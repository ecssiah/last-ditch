use ultraviolet::{IVec3, Vec3};

use crate::{
    interface::{
        constants::{BLOCK_TILE_ATLAS_HEIGHT, BLOCK_TILE_ATLAS_TILE_SIZE, BLOCK_TILE_ATLAS_WIDTH},
        gpu::gpu_texture_data::GpuTextureData,
    },
    simulation::{
        constants::CELL_RADIUS,
        state::world::{block, grid},
    },
};

pub struct TileAtlas {}

impl TileAtlas {
    pub fn get_tile_coordinates(block_kind: block::Kind, direction: grid::Direction) -> [u32; 2] {
        match block_kind {
            block::Kind::None => {
                panic!("None block has no tile data!")
            }
            block::Kind::Engraved1 => {
                let tile_coordinates_array = [[0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0]];

                tile_coordinates_array[direction as usize]
            }
            block::Kind::Engraved2 => {
                let tile_coordinates_array = [[1, 0], [1, 0], [1, 0], [1, 0], [1, 0], [1, 0]];

                tile_coordinates_array[direction as usize]
            }
            block::Kind::Stone1 => {
                let tile_coordinates_array = [[0, 1], [0, 1], [0, 1], [0, 1], [0, 1], [0, 1]];

                tile_coordinates_array[direction as usize]
            }
            block::Kind::Stone2 => {
                let tile_coordinates_array = [[1, 1], [1, 1], [1, 1], [1, 1], [1, 1], [1, 1]];

                tile_coordinates_array[direction as usize]
            }
            block::Kind::Polished1 => {
                let tile_coordinates_array = [[0, 2], [0, 2], [0, 2], [0, 2], [0, 2], [0, 2]];

                tile_coordinates_array[direction as usize]
            }
            block::Kind::Polished2 => {
                let tile_coordinates_array = [[1, 2], [1, 2], [1, 2], [1, 2], [1, 2], [1, 2]];

                tile_coordinates_array[direction as usize]
            }
            block::Kind::MagentaStone => {
                let tile_coordinates_array = [[0, 4], [0, 4], [0, 4], [0, 4], [0, 4], [0, 4]];

                tile_coordinates_array[direction as usize]
            }
            block::Kind::PurpleStone => {
                let tile_coordinates_array = [[1, 4], [1, 4], [1, 4], [1, 4], [1, 4], [1, 4]];

                tile_coordinates_array[direction as usize]
            }
            block::Kind::TealStone => {
                let tile_coordinates_array = [[2, 4], [2, 4], [2, 4], [2, 4], [2, 4], [2, 4]];

                tile_coordinates_array[direction as usize]
            }
            block::Kind::CrimsonStone => {
                let tile_coordinates_array = [[3, 4], [3, 4], [3, 4], [3, 4], [3, 4], [3, 4]];

                tile_coordinates_array[direction as usize]
            }
            block::Kind::Icon1 => {
                let tile_coordinates_array = [[0, 3], [0, 3], [0, 3], [0, 3], [0, 3], [0, 3]];

                tile_coordinates_array[direction as usize]
            }
            block::Kind::Icon2 => {
                let tile_coordinates_array = [[1, 3], [1, 3], [1, 3], [1, 3], [1, 3], [1, 3]];

                tile_coordinates_array[direction as usize]
            }
            block::Kind::Icon3 => {
                let tile_coordinates_array = [[2, 3], [2, 3], [2, 3], [2, 3], [2, 3], [2, 3]];

                tile_coordinates_array[direction as usize]
            }
            block::Kind::Icon4 => {
                let tile_coordinates_array = [[3, 3], [3, 3], [3, 3], [3, 3], [3, 3], [3, 3]];

                tile_coordinates_array[direction as usize]
            }
            block::Kind::NorthBlock => {
                let tile_coordinates_array = [[0, 5], [0, 5], [0, 5], [0, 5], [0, 5], [0, 5]];

                tile_coordinates_array[direction as usize]
            }
            block::Kind::WestBlock => {
                let tile_coordinates_array = [[1, 5], [1, 5], [1, 5], [1, 5], [1, 5], [1, 5]];

                tile_coordinates_array[direction as usize]
            }
            block::Kind::SouthBlock => {
                let tile_coordinates_array = [[2, 5], [2, 5], [2, 5], [2, 5], [2, 5], [2, 5]];

                tile_coordinates_array[direction as usize]
            }
            block::Kind::EastBlock => {
                let tile_coordinates_array = [[3, 5], [3, 5], [3, 5], [3, 5], [3, 5], [3, 5]];

                tile_coordinates_array[direction as usize]
            }
            block::Kind::EsayaBlock => {
                let tile_coordinates_array = [[0, 6], [0, 6], [0, 6], [0, 6], [0, 6], [0, 6]];

                tile_coordinates_array[direction as usize]
            }
        }
    }

    pub fn tile_coordinates_to_layer(tile_coordinates: [u32; 2]) -> u32 {
        let tiles_per_row = BLOCK_TILE_ATLAS_WIDTH / BLOCK_TILE_ATLAS_TILE_SIZE;

        tile_coordinates[1] * tiles_per_row + tile_coordinates[0]
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

    pub fn get_gpu_texture_data(
        atlas_path: &str,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> GpuTextureData {
        let img = image::open(atlas_path).expect("Failed to load atlas PNG");
        let img = img.to_rgba8();

        let (atlas_w, atlas_h) = img.dimensions();

        assert!(atlas_w % BLOCK_TILE_ATLAS_WIDTH == 0);
        assert!(atlas_h % BLOCK_TILE_ATLAS_HEIGHT == 0);

        let tiles_per_row = atlas_w / BLOCK_TILE_ATLAS_WIDTH;
        let tiles_per_col = atlas_h / BLOCK_TILE_ATLAS_HEIGHT;
        let total_tiles = tiles_per_row * tiles_per_col;

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Block Tile Texture Array"),
            size: wgpu::Extent3d {
                width: BLOCK_TILE_ATLAS_WIDTH,
                height: BLOCK_TILE_ATLAS_HEIGHT,
                depth_or_array_layers: total_tiles,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        for tile_index in 0..total_tiles {
            let tx = tile_index % tiles_per_row;
            let ty = tile_index / tiles_per_row;

            let x0 = tx * BLOCK_TILE_ATLAS_WIDTH;
            let y0 = ty * BLOCK_TILE_ATLAS_HEIGHT;

            let mut tile_pixels =
                Vec::with_capacity((BLOCK_TILE_ATLAS_WIDTH * BLOCK_TILE_ATLAS_HEIGHT * 4) as usize);

            for row in 0..BLOCK_TILE_ATLAS_HEIGHT {
                for col in 0..BLOCK_TILE_ATLAS_WIDTH {
                    let px = img.get_pixel(x0 + col, y0 + row);

                    tile_pixels.extend_from_slice(&px.0);
                }
            }

            queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: 0,
                        z: tile_index,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                &tile_pixels,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * BLOCK_TILE_ATLAS_WIDTH),
                    rows_per_image: Some(BLOCK_TILE_ATLAS_HEIGHT),
                },
                wgpu::Extent3d {
                    width: BLOCK_TILE_ATLAS_WIDTH,
                    height: BLOCK_TILE_ATLAS_HEIGHT,
                    depth_or_array_layers: 1,
                },
            );
        }

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("Tile Atlas Texture View"),
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            ..Default::default()
        });

        GpuTextureData {
            texture,
            texture_view,
            sampler,
        }
    }
}
