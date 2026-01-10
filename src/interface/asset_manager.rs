pub mod asset_status;
pub mod block_model_key;
pub mod block_texture_key;
pub mod layer_index;
pub mod model_work_result;
pub mod person_model_key;
pub mod person_texture_key;
pub mod texture_atlas;
pub mod texture_data;
pub mod texture_work_result;

use crate::{
    interface::{
        asset_manager::{
            asset_status::AssetStatus, block_model_key::BlockModelKey,
            block_texture_key::BlockTextureKey, layer_index::LayerIndex,
            model_work_result::ModelWorkResult, person_model_key::PersonModelKey,
            person_texture_key::PersonTextureKey, texture_atlas::TextureAtlas,
            texture_data::TextureData, texture_work_result::TextureWorkResult,
        },
        constants::*,
        gpu::{gpu_context::GPUContext, gpu_mesh::GpuMesh, gpu_texture_data::GpuTextureData},
        renderer::{
            population_renderer::person_renderer::{
                person_model::PersonModel, person_vertex_data::PersonVertexData,
            },
            world_renderer::block_renderer::{
                block_model::BlockModel, block_vertex_data::BlockVertexData,
            },
        },
    },
    simulation::state::{
        population::identity::{appearance::skin_tone::SkinTone, sex::Sex},
        world::block::{block_kind::BlockKind, block_shape::BlockShape},
    },
    utils::id_generator::IDGenerator,
};
use itertools::Itertools;
use obj::{load_obj, TexturedVertex};
use std::{
    collections::HashMap,
    fs::{self, DirEntry, File},
    io::BufReader,
    str::FromStr,
    thread::JoinHandle,
};

pub struct AssetManager {
    pub asset_status: AssetStatus,

    pub texture_id_generator: IDGenerator,
    pub texture_work_handle: Option<JoinHandle<TextureWorkResult>>,

    pub texture_atlas: Option<TextureAtlas>,

    pub model_id_generator: IDGenerator,
    pub model_work_handle: Option<JoinHandle<ModelWorkResult>>,

    pub block_model_gpu_mesh_map: HashMap<BlockModelKey, GpuMesh>,
    pub person_model_gpu_mesh_map: HashMap<PersonModelKey, GpuMesh>,

    pub depth_texture: wgpu::Texture,
    pub depth_texture_view: wgpu::TextureView,
}

impl AssetManager {
    pub fn new(gpu_context: &GPUContext) -> Self {
        let asset_status = AssetStatus::InitTextures;

        let texture_id_generator = IDGenerator::new();
        let texture_work_handle = None;
        let texture_atlas = None;

        let model_id_generator = IDGenerator::new();
        let model_work_handle = None;

        let depth_texture = Self::create_depth_texture(gpu_context);
        let depth_texture_view = depth_texture.create_view(&Default::default());

        let block_model_gpu_mesh_map = HashMap::new();
        let person_model_gpu_mesh_map = HashMap::new();

        Self {
            asset_status,
            texture_id_generator,
            texture_work_handle,
            texture_atlas,
            model_id_generator,
            model_work_handle,
            depth_texture,
            depth_texture_view,
            block_model_gpu_mesh_map,
            person_model_gpu_mesh_map,
        }
    }

    pub fn get_block_texture_name(block_texture_key: &BlockTextureKey) -> String {
        block_texture_key.block_kind.to_string()
    }

    pub fn get_block_texture_key(block_texture_name: &str) -> BlockTextureKey {
        BlockTextureKey::from_block_kind(
            &BlockKind::from_str(block_texture_name)
                .unwrap_or_else(|_| panic!("invalid block texture name")),
        )
    }

    pub fn get_person_texture_name(person_texture_key: &PersonTextureKey) -> String {
        person_texture_key.skin_tone.to_string()
    }

    pub fn get_person_texture_key(person_texture_name: &str) -> PersonTextureKey {
        PersonTextureKey::from_skin_tone(
            &SkinTone::from_str(person_texture_name)
                .unwrap_or_else(|_| panic!("invalid person texture name")),
        )
    }

    pub fn get_block_model_name(block_model_key: &BlockModelKey) -> String {
        block_model_key.block_shape.to_string()
    }

    pub fn get_block_model_key(block_model_name: &str) -> BlockModelKey {
        BlockModelKey::from_block_shape(
            &BlockShape::from_str(block_model_name)
                .unwrap_or_else(|_| panic!("invalid block model name")),
        )
    }

    pub fn get_person_model_name(person_model_key: &PersonModelKey) -> String {
        format!(
            "{}Age{}",
            person_model_key.sex.to_string(),
            person_model_key.age_period
        )
    }

    pub fn get_person_model_key(person_model_name: &str) -> PersonModelKey {
        let (sex_string, age_period_string) = person_model_name
            .split_once("Age")
            .expect("invalid person model name");

        let age_period: u32 = age_period_string.parse().unwrap();

        PersonModelKey::from_sex_and_age(
            &Sex::from_str(sex_string).unwrap_or_else(|_| panic!("invalid sex name")),
            age_period,
        )
    }

    pub fn get_block_layer_index(
        block_texture_key: &BlockTextureKey,
        asset_manager: &Self,
    ) -> LayerIndex {
        asset_manager
            .texture_atlas
            .as_ref()
            .expect("texture atlas needs to exist")
            .block_layer_map
            .get(block_texture_key)
            .expect("invalid block texture key")
            .clone()
    }

    pub fn get_person_layer_index(
        person_texture_key: &PersonTextureKey,
        asset_manager: &Self,
    ) -> LayerIndex {
        asset_manager
            .texture_atlas
            .as_ref()
            .expect("texture atlas needs to exist")
            .person_layer_map
            .get(person_texture_key)
            .expect("invalid person texture key")
            .clone()
    }

    pub fn init_texture_loading(asset_manager: &mut Self) {
        asset_manager.texture_work_handle = Some(std::thread::spawn(|| Self::load_texture_work()));

        asset_manager.asset_status = AssetStatus::LoadingTextures;
    }

    pub fn update_texture_loading(gpu_context: &mut GPUContext, asset_manager: &mut Self) {
        let is_finished = match &asset_manager.texture_work_handle {
            Some(handle) => handle.is_finished(),
            None => false,
        };

        if !is_finished {
            return;
        }

        let handle = asset_manager
            .texture_work_handle
            .take()
            .expect("texture_work_handle does not exist");

        let mut texture_work_result = handle.join().expect("texture loading thread failed");

        Self::commit_texture_data(
            gpu_context,
            std::mem::take(&mut texture_work_result),
            asset_manager,
        );

        asset_manager.asset_status = AssetStatus::InitModels;
    }

    fn load_texture_work() -> TextureWorkResult {
        let block_texture_directory_path = "assets/textures/block/";
        let block_texture_data_vec = Self::load_texture_data_vec(block_texture_directory_path);

        let person_texture_directory_path = "assets/textures/person/";
        let person_texture_data_vec = Self::load_texture_data_vec(person_texture_directory_path);

        let texture_work_result = TextureWorkResult {
            block_texture_data_vec,
            person_texture_data_vec,
        };

        texture_work_result
    }

    fn load_texture_data_vec(texture_directory_path: &str) -> Vec<TextureData> {
        let mut texture_data_vec = Vec::new();

        let dir_entry_vec: Vec<DirEntry> = fs::read_dir(texture_directory_path)
            .expect("failed to read texture directory")
            .map(|e| e.expect("failed to read texture directory entry"))
            .sorted_by_key(|e| e.path())
            .collect();

        for dir_entry in dir_entry_vec {
            let path = dir_entry.path();

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

            let texture_name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .expect("Invalid texture filename")
                .to_string();

            let pixel_vec: Vec<u8> = texture_image.into_raw();

            let texture_data = TextureData {
                texture_name,
                pixel_vec,
            };

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
        texture_work_result: TextureWorkResult,
        asset_manager: &mut AssetManager,
    ) {
        let texture_size = TEXTURE_SIZE as u32;

        let total_layer_count = texture_work_result.block_texture_data_vec.len() as u32
            + texture_work_result.person_texture_data_vec.len() as u32;

        let max_layers = gpu_context.device.limits().max_texture_array_layers;

        assert!(total_layer_count < max_layers);

        let atlas_texture = gpu_context.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Texture Atlas"),
            size: wgpu::Extent3d {
                width: texture_size,
                height: texture_size,
                depth_or_array_layers: total_layer_count,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

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

        let mut block_layer_map = HashMap::new();

        for (layer_index, texture_data) in texture_work_result
            .block_texture_data_vec
            .into_iter()
            .enumerate()
        {
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

            let block_texture_name = texture_data.texture_name;
            let block_texture_key = Self::get_block_texture_key(&block_texture_name);

            block_layer_map.insert(block_texture_key, LayerIndex::new(layer_index as u32));

            tracing::info!("Committed {} texture", block_texture_name);
        }

        let mut person_layer_map = HashMap::new();

        for (layer_index, texture_data) in texture_work_result
            .person_texture_data_vec
            .into_iter()
            .enumerate()
        {
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

            let person_texture_name = texture_data.texture_name;
            let person_texture_key = Self::get_person_texture_key(&person_texture_name);

            person_layer_map.insert(person_texture_key, LayerIndex::new(layer_index as u32));

            tracing::info!("Committed {} texture", person_texture_name);
        }

        let gpu_texture_data =
            GpuTextureData::new(atlas_texture, atlas_texture_view, atlas_sampler);

        let texture_atlas = TextureAtlas::new(gpu_texture_data, block_layer_map, person_layer_map);

        asset_manager.texture_atlas = Some(texture_atlas);
    }

    pub fn init_model_loading(asset_manager: &mut Self) {
        asset_manager.model_work_handle = Some(std::thread::spawn(|| Self::load_model_work()));

        asset_manager.asset_status = AssetStatus::LoadingModels;
    }

    pub fn update_model_loading(gpu_context: &GPUContext, asset_manager: &mut Self) {
        let is_finished = match &asset_manager.model_work_handle {
            Some(handle) => handle.is_finished(),
            None => false,
        };

        if !is_finished {
            return;
        }

        let model_work_handle = asset_manager
            .model_work_handle
            .take()
            .expect("texture_work_handle does not exist");

        let mut model_work_result = model_work_handle
            .join()
            .expect("texture loading thread failed");

        Self::commit_model_data(
            gpu_context,
            std::mem::take(&mut model_work_result),
            asset_manager,
        );

        asset_manager.asset_status = AssetStatus::Complete;
    }

    fn load_model_work() -> ModelWorkResult {
        let person_model_map = Self::load_person_model_map();
        let block_model_map = Self::load_block_model_map();

        ModelWorkResult {
            person_model_map,
            block_model_map,
        }
    }

    fn load_person_model_map() -> HashMap<PersonModelKey, PersonModel> {
        let mut person_model_map = HashMap::new();

        let person_models_path = std::path::Path::new("assets/models/person/");

        let dir_entry_vec: Vec<DirEntry> = fs::read_dir(person_models_path)
            .expect("failed to read person models directory")
            .map(|entry| entry.expect("failed to read person model entry"))
            .sorted_by_key(|entry| entry.path())
            .collect();

        for dir_entry in dir_entry_vec {
            let path = dir_entry.path();

            if !path.is_file() {
                continue;
            }

            if path.extension().and_then(|e| e.to_str()) != Some("obj") {
                continue;
            }

            let file_stem = path.file_stem().unwrap().to_str().unwrap();

            if let Ok(model_file) = File::open(&path) {
                let model_file_reader = BufReader::new(model_file);

                match load_obj(model_file_reader) {
                    Ok(model) => {
                        let person_model = PersonModel {
                            vertex_vec: model
                                .vertices
                                .iter()
                                .map(|vertex: &TexturedVertex| PersonVertexData {
                                    position: vertex.position,
                                    normal: vertex.normal,
                                    uv: [vertex.texture[0], 1.0 - vertex.texture[1]],
                                })
                                .collect(),
                            index_vec: model.indices,
                        };

                        let person_model_name = file_stem.to_string();
                        let person_model_key = Self::get_person_model_key(&person_model_name);

                        person_model_map.insert(person_model_key, person_model);

                        tracing::info!("{}.obj loaded", person_model_name);
                    }
                    Err(err) => {
                        tracing::error!("{:?}", err);
                    }
                }
            }
        }

        person_model_map
    }

    fn load_block_model_map() -> HashMap<BlockModelKey, BlockModel> {
        let mut block_model_map = HashMap::new();

        let block_model_path = std::path::Path::new("assets/models/block/");

        let dir_entry_vec: Vec<DirEntry> = fs::read_dir(block_model_path)
            .expect("failed to read block models directory")
            .map(|entry| entry.expect("failed to read block model entry"))
            .sorted_by_key(|entry| entry.path())
            .collect();

        for dir_entry in dir_entry_vec {
            let path = dir_entry.path();

            if !path.is_file() {
                continue;
            }

            if path.extension().and_then(|e| e.to_str()) != Some("obj") {
                continue;
            }

            let file_stem = path.file_stem().unwrap().to_str().unwrap();

            if let Ok(model_file) = File::open(&path) {
                let model_file_reader = BufReader::new(model_file);

                match load_obj(model_file_reader) {
                    Ok(model) => {
                        let block_model = BlockModel {
                            vertex_vec: model
                                .vertices
                                .iter()
                                .map(|vertex: &TexturedVertex| BlockVertexData {
                                    position: vertex.position,
                                    normal: vertex.normal,
                                    uv: [vertex.texture[0], 1.0 - vertex.texture[1]],
                                })
                                .collect(),
                            index_vec: model.indices,
                        };

                        let block_model_name = file_stem.to_string();
                        let block_model_key = Self::get_block_model_key(&block_model_name);

                        block_model_map.insert(block_model_key, block_model);

                        tracing::info!("{}", block_model_name);
                    }
                    Err(err) => {
                        tracing::error!("{:?}", err);
                    }
                }
            }
        }

        block_model_map
    }

    fn commit_model_data(
        gpu_context: &GPUContext,
        model_work_result: ModelWorkResult,
        asset_manager: &mut Self,
    ) {
        for (block_model_key, block_model) in model_work_result.block_model_map {
            let block_gpu_mesh = BlockModel::to_gpu_mesh(&block_model, &gpu_context.device);

            tracing::info!("Committed {} model", block_model_key.block_shape);

            asset_manager
                .block_model_gpu_mesh_map
                .insert(block_model_key, block_gpu_mesh);
        }

        for (person_model_key, person_model) in model_work_result.person_model_map {
            let person_gpu_mesh = PersonModel::to_gpu_mesh(&person_model, &gpu_context.device);

            tracing::info!(
                "Committed {} {} model",
                person_model_key.sex,
                person_model_key.age_period
            );

            asset_manager
                .person_model_gpu_mesh_map
                .insert(person_model_key, person_gpu_mesh);
        }
    }

    pub fn get_block_model_gpu_mesh<'a>(
        block_model_key: &BlockModelKey,
        asset_manager: &'a Self,
    ) -> &'a GpuMesh {
        asset_manager
            .block_model_gpu_mesh_map
            .get(block_model_key)
            .expect("block models should all have a gpu mesh")
    }

    pub fn get_person_model_gpu_mesh<'a>(
        person_model_key: &PersonModelKey,
        asset_manager: &'a Self,
    ) -> &'a GpuMesh {
        asset_manager
            .person_model_gpu_mesh_map
            .get(person_model_key)
            .expect("person models should all have a gpu mesh")
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
