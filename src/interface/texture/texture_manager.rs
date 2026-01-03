use crate::interface::{gpu::gpu_context::GPUContext, texture::atlas_set::AtlasSet};

pub struct TextureManager {
    pub world_atlas_set: AtlasSet,
    pub population_atlas_set: AtlasSet,
    pub depth_texture: wgpu::Texture,
    pub depth_texture_view: wgpu::TextureView,
}

impl TextureManager {
    pub fn new(gpu_context: &GPUContext) -> Self {
        let world_atlas_set = AtlasSet::new(64.0);
        let population_atlas_set = AtlasSet::new(128.0);

        let depth_texture = Self::create_depth_texture(gpu_context);
        let depth_texture_view = depth_texture.create_view(&Default::default());

        Self {
            world_atlas_set,
            population_atlas_set,
            depth_texture,
            depth_texture_view,
        }
    }

    // pub fn get_gpu_texture_data(
    //     texture_path: &str,
    //     device: &wgpu::Device,
    //     queue: &wgpu::Queue,
    // ) -> GpuTextureData {
    //     let texture_image = image::open(texture_path).expect("Failed to load atlas PNG");
    //     let texture_image = texture_image.to_rgba8();

    //     let max_layers = device.limits().max_texture_array_layers;

    //     assert!(total_tiles <= max_layers);

    //     let texture = device.create_texture(&wgpu::TextureDescriptor {
    //         label: Some("Tile Texture Array"),
    //         size: wgpu::Extent3d {
    //             width: TILE_SIZE,
    //             height: TILE_SIZE,
    //             depth_or_array_layers: total_tiles,
    //         },
    //         mip_level_count: 1,
    //         sample_count: 1,
    //         dimension: wgpu::TextureDimension::D2,
    //         format: wgpu::TextureFormat::Rgba8UnormSrgb,
    //         usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
    //         view_formats: &[],
    //     });

    //     for tile_index in 0..total_tiles {
    //         let tx = tile_index % tiles_per_row;
    //         let ty = tile_index / tiles_per_row;

    //         let x0 = tx * TILE_SIZE;
    //         let y0 = ty * TILE_SIZE;

    //         let mut tile_pixels = Vec::with_capacity((TILE_SIZE * TILE_SIZE * 4) as usize);

    //         for row in 0..TILE_SIZE {
    //             for col in 0..TILE_SIZE {
    //                 let pixel = tile_atlas_image.get_pixel(x0 + col, y0 + row);

    //                 tile_pixels.extend_from_slice(&pixel.0);
    //             }
    //         }

    //         queue.write_texture(
    //             wgpu::TexelCopyTextureInfo {
    //                 texture: &texture,
    //                 mip_level: 0,
    //                 origin: wgpu::Origin3d {
    //                     x: 0,
    //                     y: 0,
    //                     z: tile_index,
    //                 },
    //                 aspect: wgpu::TextureAspect::All,
    //             },
    //             &tile_pixels,
    //             wgpu::TexelCopyBufferLayout {
    //                 offset: 0,
    //                 bytes_per_row: Some(4 * TILE_SIZE),
    //                 rows_per_image: Some(TILE_SIZE),
    //             },
    //             wgpu::Extent3d {
    //                 width: TILE_SIZE,
    //                 height: TILE_SIZE,
    //                 depth_or_array_layers: 1,
    //             },
    //         );
    //     }

    //     let texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
    //         label: Some("Tile Atlas Texture View"),
    //         dimension: Some(wgpu::TextureViewDimension::D2Array),
    //         ..Default::default()
    //     });

    //     let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
    //         mag_filter: wgpu::FilterMode::Nearest,
    //         min_filter: wgpu::FilterMode::Nearest,
    //         mipmap_filter: wgpu::FilterMode::Nearest,
    //         address_mode_u: wgpu::AddressMode::Repeat,
    //         address_mode_v: wgpu::AddressMode::Repeat,
    //         ..Default::default()
    //     });

    //     GpuTextureData {
    //         texture,
    //         texture_view,
    //         sampler,
    //     }
    // }

    pub fn load_atlas_set(atlas_set: &AtlasSet) {

    }

    pub fn get_surface_texture(gpu_context: &GPUContext) -> wgpu::SurfaceTexture {
        let surface_texture = gpu_context
            .surface
            .get_current_texture()
            .expect("failed to acquire next swapchain texture");

        surface_texture
    }

    pub fn create_depth_texture(gpu_context: &GPUContext) -> wgpu::Texture {
        let depth_texture_descriptor = wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: gpu_context.surface_config.width,
                height: gpu_context.surface_config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        };

        gpu_context.device.create_texture(&depth_texture_descriptor)
    }
}
