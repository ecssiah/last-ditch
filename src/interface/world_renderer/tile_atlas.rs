use crate::{
    interface::{
        constants::{TILE_ATLAS_WIDTH, TILE_SIZE},
        gpu::gpu_texture_data::GpuTextureData,
    },
    simulation::state::world::{block::BlockKind, grid::Direction},
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
pub static TILE_COORDINATE_ARRAY: [[[u32; 2]; 6]; BlockKind::count()] = [
    // BlockKind::Engraved1
    [
        [0, 0], [0, 0],
        [0, 0], [0, 0],
        [0, 0], [0, 0],
    ],
    // BlockKind::Engraved2
    [
        [1, 0], [1, 0],
        [1, 0], [1, 0],
        [1, 0], [1, 0],
    ],
    // BlockKind::Engraved3
    [
        [2, 0], [2, 0],
        [2, 0], [2, 0],
        [2, 0], [2, 0],
    ],
    // BlockKind::Engraved4
    [
        [3, 0], [3, 0],
        [3, 0], [3, 0],
        [3, 0], [3, 0],
    ],
    // BlockKind::Ornate1
    [
        [8, 0], [8, 0],
        [8, 0], [8, 0],
        [8, 0], [8, 0],
    ],
    // BlockKind::Ornate2
    [
        [9, 0], [9, 0],
        [9, 0], [9, 0],
        [9, 0], [9, 0],
    ],
    // BlockKind::Ornate3
    [
        [10, 0], [10, 0],
        [10, 0], [10, 0],
        [10, 0], [10, 0],
    ],
    // BlockKind::Ornate4
    [
        [11, 0], [11, 0],
        [11, 0], [11, 0],
        [11, 0], [11, 0],
    ],
    // BlockKind::Carved1
    [
        [0, 1], [0, 1],
        [0, 1], [0, 1],
        [0, 1], [0, 1],
    ],
    // BlockKind::Carved2
    [
        [1, 1], [1, 1],
        [1, 1], [1, 1],
        [1, 1], [1, 1],
    ],
    // BlockKind::Carved3
    [
        [2, 1], [2, 1],
        [2, 1], [2, 1],
        [2, 1], [2, 1],
    ],
    // BlockKind::Carved4
    [
        [3, 1], [3, 1],
        [3, 1], [3, 1],
        [3, 1], [3, 1],
    ],
    // BlockKind::Stone1
    [
        [0, 2], [0, 2],
        [0, 2], [0, 2],
        [0, 2], [0, 2],
    ],
    // BlockKind::Stone2
    [
        [1, 2], [1, 2],
        [1, 2], [1, 2],
        [1, 2], [1, 2],
    ],
    // BlockKind::Stone3
    [
        [2, 2], [2, 2],
        [2, 2], [2, 2],
        [2, 2], [2, 2],
    ],
    // BlockKind::Stone4
    [
        [3, 2], [3, 2],
        [3, 2], [3, 2],
        [3, 2], [3, 2],
    ],
    // BlockKind::LionSymbol
    [
        [0, 3], [0, 3],
        [0, 3], [0, 3],
        [0, 3], [0, 3],
    ],
    // BlockKind::EagleSymbol
    [
        [1, 3], [1, 3],
        [1, 3], [1, 3],
        [1, 3], [1, 3],
    ],
    // BlockKind::WolfSymbol
    [
        [2, 3], [2, 3],
        [2, 3], [2, 3],
        [2, 3], [2, 3],
    ],
    // BlockKind::HorseSymbol
    [
        [3, 3], [3, 3],
        [3, 3], [3, 3],
        [3, 3], [3, 3],
    ],
    // BlockKind::LionStone
    [
        [0, 4], [0, 4],
        [0, 4], [0, 4],
        [0, 4], [0, 4],
    ],
    // BlockKind::EagleStone
    [
        [1, 4], [1, 4],
        [1, 4], [1, 4],
        [1, 4], [1, 4],
    ],
    // BlockKind::WolfStone
    [
        [2, 4], [2, 4],
        [2, 4], [2, 4],
        [2, 4], [2, 4],
    ],
    // BlockKind::HorseStone
    [
        [3, 4], [3, 4],
        [3, 4], [3, 4],
        [3, 4], [3, 4],
    ],
    // BlockKind::EastBlock
    [
        [0, 5], [0, 5],
        [0, 5], [0, 5],
        [0, 5], [0, 5],
    ],
    // BlockKind::WestBlock
    [
        [1, 5], [1, 5],
        [1, 5], [1, 5],
        [1, 5], [1, 5],
    ],
    // BlockKind::NorthBlock
    [
        [2, 5], [2, 5],
        [2, 5], [2, 5],
        [2, 5], [2, 5],
    ],
    // BlockKind::SouthBlock
    [
        [3, 5], [3, 5],
        [3, 5], [3, 5],
        [3, 5], [3, 5],
    ],
    // BlockKind::Server1
    [
        [0, 6], [0, 6],
        [0, 6], [0, 6],
        [0, 6], [0, 6],
    ],
    // BlockKind::Server2
    [
        [1, 6], [1, 6],
        [1, 6], [1, 6],
        [1, 6], [1, 6],
    ],
    // BlockKind::Server3
    [
        [2, 6], [2, 6],
        [2, 6], [2, 6],
        [2, 6], [2, 6],
    ],
    // BlockKind::Server4
    [
        [3, 6], [3, 6],
        [3, 6], [3, 6],
        [3, 6], [3, 6],
    ],
    // BlockKind::Metal1
    [
        [0, 8], [0, 8],
        [0, 8], [0, 8],
        [0, 8], [0, 8],
    ],
    // BlockKind::Metal2
    [
        [1, 8], [1, 8],
        [1, 8], [1, 8],
        [1, 8], [1, 8],
    ],
    // BlockKind::Metal3
    [
        [2, 8], [2, 8],
        [2, 8], [2, 8],
        [2, 8], [2, 8],
    ],
    // BlockKind::Metal4
    [
        [3, 8], [3, 8],
        [3, 8], [3, 8],
        [3, 8], [3, 8],
    ],
    // BlockKind::Panel1
    [
        [0, 9], [0, 9],
        [0, 9], [0, 9],
        [0, 9], [0, 9],
    ],
    // BlockKind::Panel2
    [
        [1, 9], [1, 9],
        [1, 9], [1, 9],
        [1, 9], [1, 9],
    ],
    // BlockKind::Panel3
    [
        [2, 9], [2, 9],
        [2, 9], [2, 9],
        [2, 9], [2, 9],
    ],
    // BlockKind::Panel4
    [
        [2, 9], [2, 9],
        [2, 9], [2, 9],
        [2, 9], [2, 9],
    ],
    // BlockKind::Vent1
    [
        [0, 10], [0, 10],
        [0, 10], [0, 10],
        [0, 10], [0, 10],
    ],
    // BlockKind::Vent2
    [
        [1, 10], [1, 10],
        [1, 10], [1, 10],
        [1, 10], [1, 10],
    ],
    // BlockKind::Vent3
    [
        [2, 10], [2, 10],
        [2, 10], [2, 10],
        [2, 10], [2, 10],
    ],
    // BlockKind::Vent4
    [
        [3, 10], [3, 10],
        [3, 10], [3, 10],
        [3, 10], [3, 10],
    ],
    // BlockKind::Caution1
    [
        [0, 11], [0, 11],
        [0, 11], [0, 11],
        [0, 11], [0, 11],
    ],
    // BlockKind::Caution2
    [
        [0, 11], [0, 11],
        [0, 11], [0, 11],
        [0, 11], [0, 11],
    ],
    // BlockKind::Caution3
    [
        [0, 11], [0, 11],
        [0, 11], [0, 11],
        [0, 11], [0, 11],
    ],
    // BlockKind::Caution4
    [
        [0, 11], [0, 11],
        [0, 11], [0, 11],
        [0, 11], [0, 11],
    ],
    // BlockKind::Platform1,
    [
        [0, 0], [0, 0],
        [0, 0], [0, 0],
        [0, 0], [0, 0],
    ],
    // BlockKind::Stairs1,
    [
        [0, 0], [0, 0],
        [0, 0], [0, 0],
        [0, 0], [0, 0],
    ],
    // BlockKind::Ladder1,
    [
        [0, 0], [0, 0],
        [0, 0], [0, 0],
        [0, 0], [0, 0],
    ],
    // BlockKind::Door1,
    [
        [0, 0], [0, 0],
        [0, 0], [0, 0],
        [0, 0], [0, 0],
    ],
];

pub fn get_tile_coordinate(block_kind: &BlockKind, direction: &Direction) -> [u32; 2] {
    let block_index = BlockKind::to_index(block_kind);
    let direction_index = Direction::to_index(direction);

    TILE_COORDINATE_ARRAY[block_index][direction_index]
}

pub fn tile_coordinate_to_layer(tile_coordinate: [u32; 2]) -> u32 {
    let tiles_per_row = TILE_ATLAS_WIDTH / TILE_SIZE;

    tile_coordinate[1] * tiles_per_row + tile_coordinate[0]
}

pub fn get_tile_layer(block_kind: &BlockKind, direction: &Direction) -> u32 {
    let tile_coordinate = get_tile_coordinate(block_kind, direction);

    tile_coordinate_to_layer(tile_coordinate)
}
