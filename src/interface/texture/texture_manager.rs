use itertools::Itertools;

use crate::{
    interface::{
        constants::{POPULATION_TEXTURE_SIZE, WORLD_TEXTURE_SIZE},
        gpu::gpu_context::GPUContext,
        texture::{
            texture_atlas_set::TextureAtlasSet, texture_data::TextureData,
            texture_load_status::TextureLoadStatus, texture_load_work::TextureLoadWork,
        },
    },
    simulation::utils::IDGenerator,
};
use std::{fs, thread::JoinHandle};

pub struct TextureManager {
    pub texture_id_generator: IDGenerator,
    pub texture_load_status: TextureLoadStatus,
    pub texture_load_handle: Option<JoinHandle<TextureLoadWork>>,
    pub world_texture_atlas_set: TextureAtlasSet,
    pub population_texture_atlas_set: TextureAtlasSet,
    pub depth_texture: wgpu::Texture,
    pub depth_texture_view: wgpu::TextureView,
}

impl TextureManager {
    pub fn new(gpu_context: &GPUContext) -> Self {
        let texture_id_generator = IDGenerator::new();
        let texture_load_status = TextureLoadStatus::Idle;
        let texture_load_handle = None;

        let world_texture_atlas_set = TextureAtlasSet::new("World", 64.0);
        let population_texture_atlas_set = TextureAtlasSet::new("Population", 128.0);

        let depth_texture = Self::create_depth_texture(gpu_context);
        let depth_texture_view = depth_texture.create_view(&Default::default());

        Self {
            texture_id_generator,
            texture_load_status,
            texture_load_handle,
            world_texture_atlas_set,
            population_texture_atlas_set,
            depth_texture,
            depth_texture_view,
        }
    }

    pub fn load(texture_manager: &mut Self) {
        texture_manager.texture_load_status = TextureLoadStatus::Loading;

        texture_manager.texture_load_handle =
            Some(std::thread::spawn(|| Self::load_texture_directories()))
    }

    pub fn update(gpu_context: &mut GPUContext, texture_manager: &mut Self) {
        if texture_manager.texture_load_status != TextureLoadStatus::Loading {
            return;
        }

        let is_finished = match &texture_manager.texture_load_handle {
            Some(handle) => handle.is_finished(),
            None => false,
        };

        if !is_finished {
            return;
        }

        let handle = texture_manager
            .texture_load_handle
            .take()
            .expect("texture_load_handle does not exist");

        let mut texture_load_work = handle.join().expect("texture loading thread failed");

        Self::commit(gpu_context, &mut texture_load_work, texture_manager);

        texture_manager.texture_load_status = TextureLoadStatus::Complete;
    }

    fn load_texture_directories() -> TextureLoadWork {
        let world_texture_directory_path = "assets/textures/world/";

        let world_texture_data_vec =
            Self::load_texture_data_vec(WORLD_TEXTURE_SIZE as u32, world_texture_directory_path);

        let population_texture_directory_path = "assets/textures/world/";

        let population_texture_data_vec = Self::load_texture_data_vec(
            POPULATION_TEXTURE_SIZE as u32,
            population_texture_directory_path,
        );

        TextureLoadWork {
            world_texture_data_vec,
            population_texture_data_vec,
        }
    }

    fn load_texture_data_vec(texture_size: u32, texture_directory_path: &str) -> Vec<TextureData> {
        let mut texture_data_vec = Vec::new();

        let read_dir: Vec<_> = fs::read_dir(texture_directory_path)
            .expect("world atlas textures failed to load")
            .map(|e| e.expect("world atlas entry failed"))
            .sorted_by_key(|e| e.path())
            .collect();

        for entry in read_dir {
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            if path.extension().and_then(|e| e.to_str()) != Some("png") {
                continue;
            }

            let path_str = path.to_str().expect("Invalid UTF-8 path");

            let texture_image = Self::load_image_rgba(path_str);

            let (texture_width, texture_height) = texture_image.dimensions();

            assert!(
                texture_width == texture_size && texture_height == texture_size,
                "World texture {:?} has size {}x{}, expected {}x{}",
                path,
                texture_width,
                texture_height,
                texture_size,
                texture_size,
            );

            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .expect("Invalid texture filename")
                .to_string();

            let pixel_vec: Vec<u8> = texture_image.into_raw();

            let texture_data = TextureData { name, pixel_vec };

            texture_data_vec.push(texture_data);
        }

        texture_data_vec
    }

    fn load_image_rgba(path: &str) -> image::RgbaImage {
        let texture_image = image::open(path)
            .unwrap_or_else(|e| panic!("Failed to open texture {}: {}", path, e))
            .to_rgba8();

        texture_image
    }

    pub fn commit(
        gpu_context: &mut GPUContext,
        texture_load_work: &mut TextureLoadWork,
        texture_manager: &mut Self,
    ) {
        Self::commit_texture_data(
            WORLD_TEXTURE_SIZE as u32,
            gpu_context,
            std::mem::take(&mut texture_load_work.world_texture_data_vec),
            &mut texture_manager.texture_id_generator,
            &mut texture_manager.world_texture_atlas_set,
        );

        Self::commit_texture_data(
            POPULATION_TEXTURE_SIZE as u32,
            gpu_context,
            std::mem::take(&mut texture_load_work.population_texture_data_vec),
            &mut texture_manager.texture_id_generator,
            &mut texture_manager.population_texture_atlas_set,
        );
    }

    fn commit_texture_data(
        texture_size: u32,
        gpu_context: &GPUContext,
        texture_data_vec: Vec<TextureData>,
        texture_id_generator: &mut IDGenerator,
        texture_atlas_set: &mut TextureAtlasSet,
    ) {
        let max_layers = gpu_context.device.limits().max_texture_array_layers as usize;

        for (atlas_index, atlas_chunk) in texture_data_vec.chunks(max_layers).enumerate() {
            let layer_count = atlas_chunk.len() as u32;

            let atlas_texture = gpu_context.device.create_texture(&wgpu::TextureDescriptor {
                label: Some(&format!("{} Atlas Set Texture", texture_atlas_set.name)),
                size: wgpu::Extent3d {
                    width: texture_size,
                    height: texture_size,
                    depth_or_array_layers: layer_count,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });

            for (layer_index, texture_data) in atlas_chunk.iter().enumerate() {
                gpu_context.queue.write_texture(
                    wgpu::TexelCopyTextureInfo {
                        texture: &atlas_texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d {
                            x: 0,
                            y: 0,
                            z: layer_index as u32,
                        },
                        aspect: wgpu::TextureAspect::All,
                    },
                    &texture_data.pixel_vec,
                    wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(4 * texture_size),
                        rows_per_image: Some(texture_size),
                    },
                    wgpu::Extent3d {
                        width: texture_size,
                        height: texture_size,
                        depth_or_array_layers: 1,
                    },
                );

                let texture_id = IDGenerator::allocate(texture_id_generator);

                TextureAtlasSet::insert_texture_mapping(
                    texture_id,
                    &texture_data.name,
                    atlas_index as u8,
                    layer_index as u8,
                    texture_atlas_set,
                );
            }

            let atlas_texture_view = atlas_texture.create_view(&wgpu::TextureViewDescriptor {
                dimension: Some(wgpu::TextureViewDimension::D2Array),
                ..Default::default()
            });

            let atlas_sampler = gpu_context.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                address_mode_u: wgpu::AddressMode::Repeat,
                address_mode_v: wgpu::AddressMode::Repeat,
                ..Default::default()
            });

            TextureAtlasSet::add_atlas_texture(
                &atlas_texture,
                &atlas_texture_view,
                &atlas_sampler,
                texture_atlas_set,
            );
        }
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
