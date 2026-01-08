pub mod asset_status;
pub mod block_asset;
pub mod block_key;
pub mod layer_index;
pub mod model_work_result;
pub mod person_key;
pub mod texture_atlas;
pub mod texture_data;
pub mod texture_work_result;

use crate::{
    interface::{
        asset_manager::{
            asset_status::AssetStatus, block_asset::BlockAsset, block_key::BlockKey,
            layer_index::LayerIndex, model_work_result::ModelWorkResult, person_key::PersonKey,
            texture_atlas::TextureAtlas, texture_data::TextureData,
            texture_work_result::TextureWorkResult,
        },
        constants::*,
        gpu::{gpu_context::GPUContext, gpu_texture_data::GpuTextureData},
    },
    simulation::state::{
        population::identity::appearance::skin_tone::SkinTone, world::block::block_kind::BlockKind,
    },
    utils::id_generator::IDGenerator,
};
use itertools::Itertools;
use std::{
    collections::HashMap,
    fs::{self, DirEntry},
    thread::JoinHandle,
};

pub struct AssetManager<'window> {
    pub asset_status: AssetStatus,

    pub texture_id_generator: IDGenerator,
    pub texture_work_handle: Option<JoinHandle<TextureWorkResult>>,
    pub texture_atlas: Option<TextureAtlas>,

    pub model_id_generator: IDGenerator,
    pub model_work_handle: Option<JoinHandle<ModelWorkResult>>,

    pub depth_texture: wgpu::Texture,
    pub depth_texture_view: wgpu::TextureView,

    pub block_texture_name_map: HashMap<BlockKey<'window>, &'static str>,
    pub block_model_name_map: HashMap<BlockKey<'window>, &'static str>,
    pub block_asset_map: HashMap<BlockKey<'window>, BlockAsset>,

    pub person_texture_name_map: HashMap<PersonKey<'window>, &'static str>,
    pub person_model_name_map: HashMap<PersonKey<'window>, &'static str>,
    
}

impl<'window> AssetManager<'window> {
    pub fn new(gpu_context: &GPUContext) -> Self {
        let asset_status = AssetStatus::Init;

        let texture_id_generator = IDGenerator::new();
        let texture_work_handle = None;
        let texture_atlas = None;

        let model_id_generator = IDGenerator::new();
        let model_work_handle = None;

        let depth_texture = Self::create_depth_texture(gpu_context);
        let depth_texture_view = depth_texture.create_view(&Default::default());

        let block_texture_name_map = Self::setup_block_texture_name_map();
        let block_model_name_map = Self::setup_block_model_name_map();
        let block_asset_map = Self::setup_block_asset_map();

        let person_texture_name_map = Self::setup_person_texture_name_map();
        let person_model_name_map = Self::setup_person_model_name_map();

        Self {
            asset_status,
            texture_id_generator,
            texture_work_handle,
            texture_atlas,
            model_id_generator,
            model_work_handle,
            depth_texture,
            depth_texture_view,
            block_texture_name_map,
            block_model_name_map,
            block_asset_map,
            person_texture_name_map,
            person_model_name_map,
        }
    }

    fn setup_block_texture_name_map() -> HashMap<BlockKey<'window>, &'static str> {
        let block_texture_name_map = HashMap::from([
            (BlockKey::new(&BlockKind::Engraved1), "engraved 1"),
            (BlockKey::new(&BlockKind::Engraved2), "engraved 2"),
            (BlockKey::new(&BlockKind::Engraved3), "engraved 3"),
            (BlockKey::new(&BlockKind::Engraved4), "engraved 4"),
        ]);

        block_texture_name_map
    }

    pub fn get_block_texture_name(block_kind: &BlockKind, asset_manager: &Self) -> &'static str {
        asset_manager
            .block_texture_name_map
            .get(&BlockKey::new(block_kind))
            .clone()
            .expect("All blocks should have a texture name")
    }

    fn setup_block_model_name_map() -> HashMap<BlockKey<'window>, &'static str> {
        let block_model_map = HashMap::from([
            (BlockKey::new(&BlockKind::Engraved1), "engraved 1"),
            (BlockKey::new(&BlockKind::Engraved2), "engraved 2"),
            (BlockKey::new(&BlockKind::Engraved3), "engraved 3"),
            (BlockKey::new(&BlockKind::Engraved4), "engraved 4"),
        ]);

        block_model_map
    }

    pub fn get_block_model_name_name(block_kind: &BlockKind, asset_manager: &Self) -> &'static str {
        asset_manager
            .block_model_name_map
            .get(&BlockKey::new(block_kind))
            .clone()
            .expect("All blocks should have a model name")
    }

    fn setup_block_asset_map() -> HashMap<BlockKey<'window>, BlockAsset> {
        let block_asset_map = HashMap::new();

        block_asset_map
    }

    fn setup_person_texture_name_map() -> HashMap<PersonKey<'window>, &'static str> {
        let person_texture_name_map = HashMap::from([
            (PersonKey::new(&SkinTone::Color1), "person 1"),
            (PersonKey::new(&SkinTone::Color2), "person 2"),
            (PersonKey::new(&SkinTone::Color3), "person 3"),
            (PersonKey::new(&SkinTone::Color4), "person 4"),
            (PersonKey::new(&SkinTone::Color5), "person 5"),
            (PersonKey::new(&SkinTone::Color6), "person 6"),
            (PersonKey::new(&SkinTone::Color7), "person 7"),
            (PersonKey::new(&SkinTone::Color8), "person 8"),
        ]);

        person_texture_name_map
    }

    pub fn get_person_texture_name(skin_tone: &SkinTone, asset_manager: &Self) -> &'static str {
        asset_manager
            .person_texture_name_map
            .get(&PersonKey::new(&skin_tone))
            .clone()
            .expect("All person skin tones should have a texture")
    }

    pub fn setup_person_model_name_map() -> HashMap<PersonKey<'window>, &'static str> {
        let person_model_name_map = HashMap::from([
            (PersonKey::new(&SkinTone::Color1), "person 1"),
            (PersonKey::new(&SkinTone::Color2), "person 2"),
            (PersonKey::new(&SkinTone::Color3), "person 3"),
            (PersonKey::new(&SkinTone::Color4), "person 4"),
            (PersonKey::new(&SkinTone::Color5), "person 5"),
            (PersonKey::new(&SkinTone::Color6), "person 6"),
            (PersonKey::new(&SkinTone::Color7), "person 7"),
            (PersonKey::new(&SkinTone::Color8), "person 8"),
        ]);

        person_model_name_map
    }

    pub fn get_person_model_name(skin_tone: &SkinTone, asset_manager: &Self) -> &'static str {
        asset_manager
            .person_model_name_map
            .get(&PersonKey::new(&skin_tone))
            .clone()
            .expect("All person skin tones should have a texture")
    }

    pub fn init_texture_loading(asset_manager: &mut Self) {
        asset_manager.asset_status = AssetStatus::LoadingTextures;

        asset_manager.texture_work_handle = Some(std::thread::spawn(|| Self::load_texture_work()))
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

        asset_manager.asset_status = AssetStatus::LoadingModels;
    }

    fn load_texture_work() -> TextureWorkResult {
        let block_texture_directory_path = "assets/textures/block/";
        let block_texture_data_vec = Self::load_texture_data_vec(block_texture_directory_path);

        let person_texture_directory_path = "assets/textures/person/";
        let person_texture_data_vec = Self::load_texture_data_vec(person_texture_directory_path);

        let texture_data_vec = vec![block_texture_data_vec, person_texture_data_vec]
            .into_iter()
            .flatten()
            .collect();

        let texture_work_result = TextureWorkResult { texture_data_vec };

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
        let layer_count = texture_work_result.texture_data_vec.len() as u32;
        let max_layers = gpu_context.device.limits().max_texture_array_layers as usize;

        assert!(texture_work_result.texture_data_vec.len() < max_layers);

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

        let mut name_layer_index_map = HashMap::new();

        for (layer_index, texture_data) in
            texture_work_result.texture_data_vec.into_iter().enumerate()
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

            name_layer_index_map.insert(
                texture_data.texture_name,
                LayerIndex::new(layer_index as u32),
            );
        }

        let gpu_texture_data =
            GpuTextureData::new(atlas_texture, atlas_texture_view, atlas_sampler);

        let texture_atlas = TextureAtlas::new(gpu_texture_data, name_layer_index_map);

        asset_manager.texture_atlas = Some(texture_atlas);
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

    pub fn update_model_loading(gpu_context: &GPUContext, asset_manager: &mut Self) {
        let is_finished = match &asset_manager.texture_work_handle {
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

    fn model_loading_work() {}

    fn commit_model_data(
        gpu_context: &GPUContext,
        model_work_result: ModelWorkResult,
        asset_manager: &mut Self,
    ) {

    }

    // pub fn load_person_model_map(device: &wgpu::Device, asset_manager: &mut AssetManager) {
    //     let mut person_gpu_mesh_map = HashMap::new();

    //     let person_models_path = std::path::Path::new("assets/models/person/");

    //     let dir_entry_vec: Vec<DirEntry> = fs::read_dir(person_models_path)
    //         .expect("failed to read person models directory")
    //         .map(|entry| entry.expect("failed to read person model entry"))
    //         .sorted_by_key(|entry| entry.path())
    //         .collect();

    //     for dir_entry in dir_entry_vec {
    //         let path = dir_entry.path();

    //         if !path.is_file() {
    //             continue;
    //         }

    //         if path.extension().and_then(|e| e.to_str()) != Some("obj") {
    //             continue;
    //         }

    //         let file_stem = path.file_stem().unwrap().to_str().unwrap();

    //         if let Ok(model_file) = File::open(&path) {
    //             let model_file_reader = BufReader::new(model_file);

    //             match load_obj(model_file_reader) {
    //                 Ok(model) => {
    //                     let person_mesh = PersonModel {
    //                         vertex_vec: model
    //                             .vertices
    //                             .iter()
    //                             .map(|vertex: &TexturedVertex| PersonVertexData {
    //                                 position: vertex.position,
    //                                 normal: vertex.normal,
    //                                 uv: [vertex.texture[0], 1.0 - vertex.texture[1]],
    //                             })
    //                             .collect(),
    //                         index_vec: model.indices,
    //                     };

    //                     let person_gpu_mesh_arc =
    //                         Arc::new(PersonModel::to_gpu_mesh(&person_mesh, device));

    //                     person_gpu_mesh_map.insert(file_stem.to_string(), person_gpu_mesh_arc);

    //                     tracing::info!("{}.obj loaded", file_stem);
    //                 }
    //                 Err(err) => {
    //                     tracing::error!("{:?}", err);
    //                 }
    //             }
    //         }
    //     }

    //     asset_manager.person_gpu_mesh_map = person_gpu_mesh_map;
    // }
}
