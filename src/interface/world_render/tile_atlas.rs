use crate::{
    interface::{
        constants::{TILE_ATLAS_WIDTH, TILE_SIZE},
        gpu::gpu_texture_data::GpuTextureData,
    },
    simulation::state::world::{block, grid},
};

pub fn get_gpu_texture_data(
    atlas_path: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> GpuTextureData {
    let tile_atlas_image = image::open(atlas_path).expect("Failed to load atlas PNG");
    let tile_atlas_image = tile_atlas_image.to_rgba8();

    let (atlas_width, atlas_height) = tile_atlas_image.dimensions();

    assert!(atlas_width % TILE_SIZE == 0);
    assert!(atlas_height % TILE_SIZE == 0);

    let tiles_per_row = atlas_width / TILE_SIZE;
    let tiles_per_col = atlas_height / TILE_SIZE;
    let total_tiles = tiles_per_row * tiles_per_col;

    let max_layers = device.limits().max_texture_array_layers;

    assert!(total_tiles <= max_layers);

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Tile Texture Array"),
        size: wgpu::Extent3d {
            width: TILE_SIZE,
            height: TILE_SIZE,
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

        let x0 = tx * TILE_SIZE;
        let y0 = ty * TILE_SIZE;

        let mut tile_pixels = Vec::with_capacity((TILE_SIZE * TILE_SIZE * 4) as usize);

        for row in 0..TILE_SIZE {
            for col in 0..TILE_SIZE {
                let pixel = tile_atlas_image.get_pixel(x0 + col, y0 + row);

                tile_pixels.extend_from_slice(&pixel.0);
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
                bytes_per_row: Some(4 * TILE_SIZE),
                rows_per_image: Some(TILE_SIZE),
            },
            wgpu::Extent3d {
                width: TILE_SIZE,
                height: TILE_SIZE,
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
        address_mode_u: wgpu::AddressMode::Repeat,
        address_mode_v: wgpu::AddressMode::Repeat,
        ..Default::default()
    });

    GpuTextureData {
        texture,
        texture_view,
        sampler,
    }
}

#[rustfmt::skip]
    pub fn get_tile_coordinate(block_kind: block::Kind, direction: grid::Direction) -> [u32; 2] {
        match block_kind {
            block::Kind::None => {
                panic!("None block has no tile data!")
            }
            block::Kind::Engraved1 => {
                let tile_coordinate_array = [
                    [0, 0],
                    [0, 0],
                    [0, 0],
                    [0, 0],
                    [0, 0],
                    [0, 0]
                ];

                tile_coordinate_array[direction as usize]
            }
            block::Kind::Engraved2 => {
                let tile_coordinate_array = [
                    [1, 0],
                    [1, 0],
                    [1, 0],
                    [1, 0],
                    [1, 0],
                    [1, 0]
                ];

                tile_coordinate_array[direction as usize]
            }
            block::Kind::Stone1 => {
                let tile_coordinate_array = [
                    [0, 1],
                    [0, 1],
                    [0, 1],
                    [0, 1],
                    [0, 1],
                    [0, 1]
                ];

                tile_coordinate_array[direction as usize]
            }
            block::Kind::Stone2 => {
                let tile_coordinate_array = [
                    [1, 1],
                    [1, 1],
                    [1, 1],
                    [1, 1],
                    [1, 1],
                    [1, 1]
                ];

                tile_coordinate_array[direction as usize]
            }
            block::Kind::Polished1 => {
                let tile_coordinate_array = [
                    [0, 2],
                    [0, 2],
                    [0, 2],
                    [0, 2],
                    [0, 2],
                    [0, 2]
                ];

                tile_coordinate_array[direction as usize]
            }
            block::Kind::Polished2 => {
                let tile_coordinate_array = [
                    [1, 2],
                    [1, 2],
                    [1, 2],
                    [1, 2],
                    [1, 2],
                    [1, 2]
                ];

                tile_coordinate_array[direction as usize]
            }
            block::Kind::LionStone => {
                let tile_coordinate_array = [
                    [0, 4],
                    [0, 4],
                    [0, 4],
                    [0, 4],
                    [0, 4],
                    [0, 4]
                ];

                tile_coordinate_array[direction as usize]
            }
            block::Kind::EagleStone => {
                let tile_coordinate_array = [
                    [1, 4],
                    [1, 4],
                    [1, 4],
                    [1, 4],
                    [1, 4],
                    [1, 4]
                ];

                tile_coordinate_array[direction as usize]
            }
            block::Kind::WolfStone => {
                let tile_coordinate_array = [
                    [2, 4],
                    [2, 4],
                    [2, 4],
                    [2, 4],
                    [2, 4],
                    [2, 4]
                ];

                tile_coordinate_array[direction as usize]
            }
            block::Kind::HorseStone => {
                let tile_coordinate_array = [
                    [3, 4],
                    [3, 4],
                    [3, 4],
                    [3, 4],
                    [3, 4],
                    [3, 4]
                ];

                tile_coordinate_array[direction as usize]
            }
            block::Kind::Lion => {
                let tile_coordinate_array = [
                    [0, 3],
                    [0, 3],
                    [0, 3],
                    [0, 3],
                    [0, 3],
                    [0, 3]
                ];

                tile_coordinate_array[direction as usize]
            }
            block::Kind::Eagle => {
                let tile_coordinate_array = [
                    [1, 3],
                    [1, 3],
                    [1, 3],
                    [1, 3],
                    [1, 3],
                    [1, 3]
                ];

                tile_coordinate_array[direction as usize]
            }
            block::Kind::Wolf => {
                let tile_coordinate_array = [
                    [2, 3],
                    [2, 3],
                    [2, 3],
                    [2, 3],
                    [2, 3],
                    [2, 3]
                ];

                tile_coordinate_array[direction as usize]
            }
            block::Kind::Horse => {
                let tile_coordinate_array = [
                    [3, 3],
                    [3, 3],
                    [3, 3],
                    [3, 3],
                    [3, 3],
                    [3, 3]
                ];

                tile_coordinate_array[direction as usize]
            }
            block::Kind::NorthBlock => {
                let tile_coordinate_array = [
                    [0, 5],
                    [0, 5],
                    [0, 5],
                    [0, 5],
                    [0, 5],
                    [0, 5]
                ];

                tile_coordinate_array[direction as usize]
            }
            block::Kind::WestBlock => {
                let tile_coordinate_array = [
                    [1, 5],
                    [1, 5],
                    [1, 5],
                    [1, 5],
                    [1, 5],
                    [1, 5]
                ];

                tile_coordinate_array[direction as usize]
            }
            block::Kind::SouthBlock => {
                let tile_coordinate_array = [
                    [2, 5],
                    [2, 5],
                    [2, 5],
                    [2, 5],
                    [2, 5],
                    [2, 5]
                ];

                tile_coordinate_array[direction as usize]
            }
            block::Kind::EastBlock => {
                let tile_coordinate_array = [
                    [3, 5],
                    [3, 5],
                    [3, 5],
                    [3, 5],
                    [3, 5],
                    [3, 5]
                ];

                tile_coordinate_array[direction as usize]
            }
            block::Kind::ServerBlock1 => {
                let tile_coordinate_array = [
                    [0, 6],
                    [0, 6],
                    [0, 6],
                    [0, 6],
                    [0, 6],
                    [0, 6]
                ];

                tile_coordinate_array[direction as usize]
            }
            block::Kind::ServerBlock2 => {
                let tile_coordinate_array = [
                    [1, 6],
                    [1, 6],
                    [1, 6],
                    [1, 6],
                    [1, 6],
                    [1, 6]
                ];

                tile_coordinate_array[direction as usize]
            }
        }
    }

pub fn tile_coordinate_to_layer(tile_coordinate: [u32; 2]) -> u32 {
    let tiles_per_row = TILE_ATLAS_WIDTH / TILE_SIZE;

    tile_coordinate[1] * tiles_per_row + tile_coordinate[0]
}

pub fn get_tile_layer(block_kind: block::Kind, direction: grid::Direction) -> u32 {
    let tile_coordinate = get_tile_coordinate(block_kind, direction);

    tile_coordinate_to_layer(tile_coordinate)
}
