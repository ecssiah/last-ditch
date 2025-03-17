use crate::{
    include_shader_src,
    interface::{self, ASPECT_RATIO, FAR_PLANE, FOV, NEAR_PLANE},
    simulation::{
        self,
        agent::Agent,
        block::{self, Neighbors},
        world::World,
        Simulation, BLOCKS, CHUNK_VOLUME,
    },
};
use bytemuck::{Pod, Zeroable};
use glam::{IVec3, Mat4, Vec3};
use std::sync::{Arc, RwLock};
use wgpu::{util::DeviceExt, vertex_attr_array};
use winit::{event::WindowEvent, window::Window};

const CLEAR_COLOR: wgpu::Color = wgpu::Color {
    r: 0.2,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::from_cols_array(&[
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
]);

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct BlockInstance {
    position: [f32; 4],
    color: [f32; 4],
    ao_1: [f32; 4],
    ao_2: [f32; 4],
}

pub struct Render {
    last_render: u64,
    window: Arc<Window>,
    agent: Arc<RwLock<Agent>>,
    world: Arc<RwLock<World>>,
    chunks: Arc<[Arc<RwLock<simulation::Chunk>>]>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
    surface_config: wgpu::SurfaceConfiguration,
    view_projection_buffer: wgpu::Buffer,
    view_projection_bind_group: wgpu::BindGroup,
    block_chunks: [interface::Chunk; CHUNK_VOLUME as usize],
    block_pipeline: wgpu::RenderPipeline,
}

impl Render {
    pub async fn new(
        window: Arc<Window>,
        agent: Arc<RwLock<Agent>>,
        world: Arc<RwLock<World>>,
        chunks: Arc<[Arc<RwLock<simulation::Chunk>>]>,
    ) -> Render {
        window.set_cursor_visible(false);

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap();

        let size = window.inner_size();

        let surface = instance.create_surface(window.clone()).unwrap();
        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities.formats[0];
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            view_formats: vec![surface_format],
            alpha_mode: wgpu::CompositeAlphaMode::PostMultiplied,
            width: size.width,
            height: size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };

        surface.configure(&device, &surface_config);

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Uniform Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let view_projection_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("View Projection Buffer"),
            size: std::mem::size_of::<[[f32; 4]; 4]>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let view_projection_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("View Projection Bind Group"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: view_projection_buffer.as_entire_binding(),
            }],
        });

        let block_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Block Shader"),
            source: wgpu::ShaderSource::Wgsl(include_shader_src!("block.wgsl").into()),
        });

        let block_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Block Pipeline Layout"),
                bind_group_layouts: &[&uniform_bind_group_layout],
                push_constant_ranges: &[],
            });

        let block_instance_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<BlockInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &vertex_attr_array![
                0 => Float32x4,
                1 => Float32x4,
                2 => Float32x4,
                3 => Float32x4,
            ],
        };

        let block_chunks = core::array::from_fn(|_| {
            let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Empty Buffer"),
                size: std::mem::size_of::<BlockInstance>() as u64,
                usage: wgpu::BufferUsages::VERTEX,
                mapped_at_creation: false,
            });

            interface::Chunk {
                last_render: 0,
                instance_buffer,
                instance_count: 0,
            }
        });

        let block_pipeline = Render::create_pipeline(
            &device,
            &surface_config,
            &block_shader,
            block_pipeline_layout,
            block_instance_layout,
        );

        let last_render: u64 = 0;

        let render = Render {
            last_render,
            window,
            agent,
            world,
            chunks,
            device,
            queue,
            size,
            surface,
            surface_format,
            surface_config,
            view_projection_buffer,
            view_projection_bind_group,
            block_chunks,
            block_pipeline,
        };

        render
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;

        self.surface.configure(&self.device, &self.surface_config);
    }

    fn update(&mut self) {
        self.update_view_projection();

        let last_update = self.world.read().unwrap().last_update;

        if last_update > self.last_render {
            self.update_chunks();

            self.last_render = last_update;
        }
    }

    pub fn render(&self) {
        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("failed to acquire next swapchain texture");

        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                format: Some(self.surface_format.add_srgb_suffix()),
                ..Default::default()
            });

        let depth_texture_view = Render::create_depth_texture(&self.device, &self.surface_config);

        let mut encoder = self.device.create_command_encoder(&Default::default());

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("World Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(CLEAR_COLOR),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &depth_texture_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&self.block_pipeline);
        render_pass.set_bind_group(0, &self.view_projection_bind_group, &[]);

        for chunk in self.block_chunks.iter() {
            if chunk.instance_count > 0 {
                render_pass.set_vertex_buffer(0, chunk.instance_buffer.slice(..));

                render_pass.draw(0..block::BLOCK_VERTEX_COUNT, 0..chunk.instance_count);
            }
        }

        drop(render_pass);

        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();

        surface_texture.present();
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::RedrawRequested => {
                self.update();
                self.render();

                self.window.request_redraw();
            }
            WindowEvent::Resized(size) => {
                self.resize(*size);
            }
            _ => (),
        }
    }

    fn update_chunks(&mut self) {
        for (chunk_id, chunk) in self.chunks.iter().enumerate() {
            let last_update = chunk.read().unwrap().last_update;

            if self.block_chunks[chunk_id].last_render < last_update {
                let mut block_instances: Vec<BlockInstance> = Vec::new();

                let chunk = chunk.read().unwrap();

                for block_id in 0..CHUNK_VOLUME as usize {
                    let palette_id = chunk.palette_ids[block_id] as usize;
                    let kind = chunk.palette[palette_id];

                    if kind != block::Kind::Air {
                        let grid_position =
                            Simulation::get_grid_position(chunk_id as u32, block_id as u32);
                        let block = &BLOCKS[kind as usize];
                        let meta = &chunk.meta[block_id];

                        let block_instance = self.create_block_instance(grid_position, block, meta);

                        block_instances.push(block_instance);
                    }
                }

                let block_instance_count = block_instances.len() as u32;

                if block_instance_count > 0 {
                    self.block_chunks[chunk_id].instance_buffer =
                        self.device
                            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                label: Some("BlockInstance Buffer"),
                                contents: bytemuck::cast_slice(block_instances.as_slice()),
                                usage: wgpu::BufferUsages::VERTEX,
                            });
                }

                self.block_chunks[chunk_id].instance_count = block_instance_count;
                self.block_chunks[chunk_id].last_render = last_update;
            }
        }
    }

    fn create_block_instance(
        &self,
        grid_position: IVec3,
        block: &block::Block,
        meta: &block::Meta,
    ) -> BlockInstance {
        let position = [
            grid_position.x as f32,
            grid_position.y as f32,
            grid_position.z as f32,
            0.0,
        ];

        let color = [
            block.color.0 as f32,
            block.color.1 as f32,
            block.color.2 as f32,
            block.color.3 as f32,
        ];

        let (ao_1, ao_2) = Render::compute_ao(meta.neighbors);

        BlockInstance {
            position,
            color,
            ao_1,
            ao_2,
        }
    }

    fn compute_ao(neighbors: block::Neighbors) -> ([f32; 4], [f32; 4]) {
        let ao_1 = [
            Render::compute_vertex_ao(
                neighbors,
                Neighbors::SED,
                (
                    Neighbors::CEC,
                    Neighbors::SCC,
                    Neighbors::CCD,
                ),
                (Neighbors::CED, Neighbors::SCD, Neighbors::SEC),
            ),
            Render::compute_vertex_ao(
                neighbors,
                Neighbors::SWD,
                (
                    Neighbors::CWC,
                    Neighbors::SCC,
                    Neighbors::CCD,
                ),
                (Neighbors::CWD, Neighbors::SCD, Neighbors::SWC),
            ),
            Render::compute_vertex_ao(
                neighbors,
                Neighbors::SEU,
                (
                    Neighbors::CEC,
                    Neighbors::SCC,
                    Neighbors::CCU,
                ),
                (Neighbors::CEU, Neighbors::SCU, Neighbors::SEC),
            ),
            Render::compute_vertex_ao(
                neighbors,
                Neighbors::SWU,
                (
                    Neighbors::CWC,
                    Neighbors::SCC,
                    Neighbors::CCU,
                ),
                (Neighbors::CWU, Neighbors::SCU, Neighbors::SWC),
            ),
        ];

        let ao_2 = [
            Render::compute_vertex_ao(
                neighbors,
                Neighbors::NED,
                (
                    Neighbors::CEC,
                    Neighbors::NCC,
                    Neighbors::CCD,
                ),
                (Neighbors::CED, Neighbors::NCD, Neighbors::NEC),
            ),
            Render::compute_vertex_ao(
                neighbors,
                Neighbors::NWD,
                (
                    Neighbors::CWC,
                    Neighbors::NCC,
                    Neighbors::CCD,
                ),
                (Neighbors::CWD, Neighbors::NCD, Neighbors::NWC),
            ),
            Render::compute_vertex_ao(
                neighbors,
                Neighbors::CCU,
                (
                    Neighbors::CEC,
                    Neighbors::NCC,
                    Neighbors::CCU,
                ),
                (Neighbors::CEU, Neighbors::NCU, Neighbors::NEC),
            ),
            Render::compute_vertex_ao(
                neighbors,
                Neighbors::CCU,
                (
                    Neighbors::CWC,
                    Neighbors::NCC,
                    Neighbors::CCU,
                ),
                (Neighbors::CWU, Neighbors::NCU, Neighbors::NWC),
            ),
        ];

        (ao_1, ao_2)
    }

    fn compute_vertex_ao(
        mask: block::Neighbors,
        point: Neighbors,
        faces: (Neighbors, Neighbors, Neighbors),
        edges: (Neighbors, Neighbors, Neighbors),
    ) -> f32 {
        let point = mask.is_solid(point) as u8 as f32;
    
        let face0 = mask.is_solid(faces.0) as u8 as f32;
        let face1 = mask.is_solid(faces.1) as u8 as f32;
        let face2 = mask.is_solid(faces.2) as u8 as f32;
    
        let edge0 = mask.is_solid(edges.0) as u8 as f32;
        let edge1 = mask.is_solid(edges.1) as u8 as f32;
        let edge2 = mask.is_solid(edges.2) as u8 as f32;
    
        if (face0 + face1) == 2.0 || (face1 + face2) == 2.0 || (face2 + face0) == 2.0 {
            return 0.5;
        }
    
        let mut occlusion = 0.0;

        occlusion += face0 * 0.30;
        occlusion += face1 * 0.30;
        occlusion += face2 * 0.30;
        
        occlusion += edge0 * 0.15;
        occlusion += edge1 * 0.15;
        occlusion += edge2 * 0.15;
    
        occlusion += point * 0.10;
    
        (1.0 - occlusion).max(0.0)
    }

    fn update_view_projection(&mut self) {
        let view_projection_matrix = Render::create_view_projection_matrix(self.agent.clone());

        self.queue.write_buffer(
            &self.view_projection_buffer,
            0,
            bytemuck::cast_slice(&view_projection_matrix),
        );
    }

    fn create_depth_texture(
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

    fn create_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        shader: &wgpu::ShaderModule,
        pipeline_layout: wgpu::PipelineLayout,
        instance_layout: wgpu::VertexBufferLayout<'_>,
    ) -> wgpu::RenderPipeline {
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("block Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: Some("vs_main"),
                buffers: &[instance_layout],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        })
    }

    fn create_view_projection_matrix(agent: Arc<RwLock<Agent>>) -> [[f32; 4]; 4] {
        let agent = agent.read().unwrap();

        let opengl_projection =
            Mat4::perspective_rh(FOV.to_radians(), ASPECT_RATIO, NEAR_PLANE, FAR_PLANE);
        let projection = OPENGL_TO_WGPU_MATRIX * opengl_projection;

        let forward = agent.look_rotation * Vec3::Z;
        let up = agent.look_rotation * Vec3::Y;

        let eye = agent.position;
        let target = eye + forward;

        let view = Mat4::look_at_rh(eye, target, up);

        let view_projection = projection * view;

        view_projection.to_cols_array_2d()
    }
}
