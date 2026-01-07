use crate::{
    interface::{
        constants::*,
        gpu::gpu_context::GPUContext,
        renderer::texture::{
            texture_atlas_set::TextureAtlasSet, texture_data::TextureData, texture_id::TextureID,
            texture_load_status::TextureLoadStatus, texture_location::TextureLocation,
        },
    },
    utils::id_generator::IDGenerator,
};
use itertools::Itertools;
use std::{fs, thread::JoinHandle};

pub struct TextureManager {
    pub texture_id_generator: IDGenerator,
    pub texture_load_status: TextureLoadStatus,
    pub texture_load_handle: Option<JoinHandle<Vec<TextureData>>>,
    pub texture_atlas_set: TextureAtlasSet,
    pub depth_texture: wgpu::Texture,
    pub depth_texture_view: wgpu::TextureView,
}

impl TextureManager {
    pub fn new(gpu_context: &GPUContext) -> Self {
        let texture_id_generator = IDGenerator::new();
        let texture_load_status = TextureLoadStatus::Idle;
        let texture_load_handle = None;

        let texture_atlas_set = TextureAtlasSet::new();

        let depth_texture = Self::create_depth_texture(gpu_context);
        let depth_texture_view = depth_texture.create_view(&Default::default());

        Self {
            texture_id_generator,
            texture_load_status,
            texture_load_handle,
            texture_atlas_set,
            depth_texture,
            depth_texture_view,
        }
    }

    pub fn get_texture_location(
        texture_name: &'static str,
        texture_manager: &Self,
    ) -> Option<TextureLocation> {
        TextureAtlasSet::get_texture_location(texture_name, &texture_manager.texture_atlas_set)
            .cloned()
    }

    pub fn load(texture_manager: &mut Self) {
        texture_manager.texture_load_status = TextureLoadStatus::Loading;

        texture_manager.texture_load_handle = Some(std::thread::spawn(|| Self::load_textures()))
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

        let mut texture_data_vec = handle.join().expect("texture loading thread failed");

        Self::commit_texture_data(
            gpu_context,
            std::mem::take(&mut texture_data_vec),
            &mut texture_manager.texture_id_generator,
            &mut texture_manager.texture_atlas_set,
        );

        texture_manager.texture_load_status = TextureLoadStatus::Complete;
    }

    fn load_textures() -> Vec<TextureData> {
        let block_texture_directory_path = "assets/textures/block/";
        let block_texture_data_vec = Self::load_texture_data_vec(block_texture_directory_path);

        let person_texture_directory_path = "assets/textures/person/";
        let person_texture_data_vec = Self::load_texture_data_vec(person_texture_directory_path);

        let structure_texture_directory_path = "assets/textures/structure/";
        let structure_texture_data_vec =
            Self::load_texture_data_vec(structure_texture_directory_path);

        let texture_data_vec = vec![
            block_texture_data_vec,
            person_texture_data_vec,
            structure_texture_data_vec,
        ];

        texture_data_vec.into_iter().flatten().collect()
    }

    fn load_texture_data_vec(texture_directory_path: &str) -> Vec<TextureData> {
        let mut texture_data_vec = Vec::new();

        let read_dir: Vec<_> = fs::read_dir(texture_directory_path)
            .expect("failed to read texture directory")
            .map(|e| e.expect("failed to read texture directory entry"))
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

            let texture_size = TEXTURE_SIZE as u32;
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

    fn commit_texture_data(
        gpu_context: &GPUContext,
        texture_data_vec: Vec<TextureData>,
        texture_id_generator: &mut IDGenerator,
        texture_atlas_set: &mut TextureAtlasSet,
    ) {
        let texture_size = TEXTURE_SIZE as u32;
        let max_layers = gpu_context.device.limits().max_texture_array_layers as usize;

        for (atlas_index, atlas_chunk) in texture_data_vec.chunks(max_layers).enumerate() {
            let layer_count = atlas_chunk.len() as u32;

            let atlas_texture = gpu_context.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("Texture Atlas"),
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

                let texture_id = TextureID::new(IDGenerator::allocate(texture_id_generator));
                let texture_location = TextureLocation::new(atlas_index, layer_index);

                TextureAtlasSet::insert_texture_mapping(
                    &texture_id,
                    &texture_data.name,
                    &texture_location,
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
