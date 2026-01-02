pub mod texture_id;
pub mod texture_location;
pub mod texture_manager;

use crate::interface::{constants::TILE_SIZE, gpu::gpu_texture_data::GpuTextureData};

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
