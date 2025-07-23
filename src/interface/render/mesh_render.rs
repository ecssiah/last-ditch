use crate::{
    interface::{
        gpu_context::GPUContext,
        render::data::{MeshData, RenderData, TextureData},
    },
    simulation::state::population::entity,
};
use obj::load_obj;
use std::{collections::HashMap, fs::File, io::BufReader};

#[derive(PartialEq, Eq, Hash)]
pub enum RenderType {
    Block,
    Entity,
    Item,
}

pub struct MeshRender {
    pub mesh_data_map: HashMap<entity::Kind, MeshData>,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    pub texture_bind_group_map: HashMap<String, wgpu::BindGroup>,
    pub render_data_map: HashMap<RenderType, Vec<RenderData>>,
}

impl MeshRender {
    pub fn new(gpu_context: &GPUContext) -> Self {
        let mesh_data_map = Self::load_mesh_data_map(&gpu_context.device);
        let texture_bind_group_layout = Self::create_texture_bind_group_layout(&gpu_context.device);
        let texture_bind_group_map =
            Self::load_texture_bind_group_map(&gpu_context.device, &gpu_context.queue);

        let render_data_map = HashMap::from([
            (RenderType::Block, Vec::new()),
            (RenderType::Entity, Vec::new()),
            (RenderType::Item, Vec::new()),
        ]);

        Self {
            mesh_data_map,
            texture_bind_group_layout,
            texture_bind_group_map,
            render_data_map,
        }
    }

    fn load_mesh_data_map(device: &wgpu::Device) -> HashMap<entity::Kind, MeshData> {
        let mut mesh_data_map = HashMap::new();

        let entity_models_path = std::path::Path::new("assets/models/entity");

        let mut entity_model_entries =
            std::fs::read_dir(entity_models_path).expect("Failed to read entity models directory");

        while let Some(Ok(entity_model_entry)) = entity_model_entries.next() {
            let path = entity_model_entry.path();

            if path.extension().and_then(|extension| extension.to_str()) == Some("obj") {
                let file_stem = path.file_stem().unwrap().to_str().unwrap();

                if let Ok(model_file) = File::open(&path) {
                    let model_file_reader = BufReader::new(model_file);

                    match load_obj(model_file_reader) {
                        Ok(model) => {
                            let vertex_vec = model.vertices;
                            let index_vec = model.indices;

                            let mesh_data = MeshData::new(device, vertex_vec, index_vec);

                            if let Some(kind) = entity::Kind::from_string(file_stem) {
                                mesh_data_map.insert(kind, mesh_data);
                            }
                        }
                        Err(err) => {
                            log::error!("{:?}", err);
                        }
                    }
                }
            }
        }

        mesh_data_map
    }

    fn create_texture_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Texture BindGroupLayout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        })
    }

    fn load_texture_bind_group_map(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> HashMap<String, wgpu::BindGroup> {
        let mut texture_bind_group_map = HashMap::new();

        let entity_textures_path = std::path::Path::new("assets/textures/entity");

        let mut entity_texture_entries_itr = std::fs::read_dir(entity_textures_path)
            .expect("Failed to read entity models directory");

        while let Some(Ok(entity_texture_entry)) = entity_texture_entries_itr.next() {
            let path = entity_texture_entry.path();

            if path.extension().and_then(|extension| extension.to_str()) == Some("png") {
                let file_stem = path.file_stem().unwrap().to_str().unwrap();

                let texture_data = pollster::block_on(Self::load_texture_data(
                    device,
                    queue,
                    path.to_str().unwrap(),
                    file_stem,
                ));

                let texture_bind_group = Self::create_texture_bind_group(device, &texture_data);

                texture_bind_group_map.insert(file_stem.to_string(), texture_bind_group);
            }
        }

        texture_bind_group_map
    }

    pub fn render(
        render_data_map: &HashMap<RenderType, Vec<RenderData>>,
        render_pass: &mut wgpu::RenderPass<'_>,
    ) {
        for render_data_vec in render_data_map.values() {
            for render_data in render_data_vec {
                render_pass.set_bind_group(1, &render_data.texture_bind_group, &[]);

                render_pass.set_vertex_buffer(0, render_data.mesh_data.vertex_buffer.slice(..));

                render_pass.set_index_buffer(
                    render_data.mesh_data.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );

                render_pass.draw_indexed(0..render_data.mesh_data.index_count, 0, 0..1);
            }
        }
    }

    pub fn create_texture_bind_group(
        device: &wgpu::Device,
        texture_data: &TextureData,
    ) -> wgpu::BindGroup {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Texture and Sampler Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_data.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture_data.sampler),
                },
            ],
        })
    }

    pub fn create_depth_texture(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        depth_texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    pub async fn load_texture_data(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        path: &str,
        label: &str,
    ) -> TextureData {
        let img = image::open(path)
            .expect("Failed to open texture atlas")
            .into_rgba8();

        let (width, height) = img.dimensions();

        let texture_size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &img,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            texture_size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("texture_atlas_sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        TextureData {
            texture,
            view,
            sampler,
        }
    }
}
