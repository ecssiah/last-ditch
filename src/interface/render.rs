use super::{ASPECT_RATIO, FAR_PLANE, FOV, NEAR_PLANE};
use crate::{
    include_shader_src,
    simulation::{
        agent::Agent, block::{Block, BlockKind}, chunk::Chunk, world::World, Simulation, BLOCKS, CHUNK_VOLUME
    },
};
use bytemuck::{Pod, Zeroable};
use glam::{IVec3, Mat4, Vec3};
use std::sync::{Arc, RwLock};
use wgpu::util::DeviceExt;
use winit::{event::WindowEvent, window::Window};

const VOXEL_INSTANCE_LAYOUT: wgpu::VertexBufferLayout = wgpu::VertexBufferLayout {
    array_stride: std::mem::size_of::<VoxelInstance>() as wgpu::BufferAddress,
    step_mode: wgpu::VertexStepMode::Instance,
    attributes: &[
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x3,
            offset: 0,
            shader_location: 0,
        },
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x4,
            offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
            shader_location: 1,
        },
    ],
};

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::from_cols_array(&[
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
]);

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct VoxelInstance {
    position: [f32; 3],
    color: [f32; 4],
}

pub struct VoxelRender {
    instance_count: u32,
    instance_buffer: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
}

pub struct Render {
    window: Arc<Window>,
    agent: Arc<RwLock<Agent>>,
    world: Arc<RwLock<World>>,
    chunks: Arc<[Arc<RwLock<Chunk>>]>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
    surface_config: wgpu::SurfaceConfiguration,
    view_projection_buffer: wgpu::Buffer,
    view_projection_bind_group: wgpu::BindGroup,
    voxel_render: VoxelRender,
}

impl Render {
    pub async fn new(
        window: Arc<Window>,
        agent: Arc<RwLock<Agent>>,
        world: Arc<RwLock<World>>,
        chunks: Arc<[Arc<RwLock<Chunk>>]>,
    ) -> Render {
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

        let voxel_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Voxel Shader"),
            source: wgpu::ShaderSource::Wgsl(include_shader_src!("voxel.wgsl").into()),
        });

        let voxel_instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Empty Buffer"),
            size: 4,
            usage: wgpu::BufferUsages::VERTEX,
            mapped_at_creation: false,
        });

        let voxel_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Voxel Pipeline Layout"),
                bind_group_layouts: &[&uniform_bind_group_layout],
                push_constant_ranges: &[],
            });

        let voxel_pipeline = Render::create_pipeline(
            &device,
            &surface_config,
            &voxel_shader,
            voxel_pipeline_layout,
            VOXEL_INSTANCE_LAYOUT,
        );

        let voxel_render = VoxelRender {
            instance_count: 0,
            instance_buffer: voxel_instance_buffer,
            pipeline: voxel_pipeline,
        };

        let render = Render {
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
            voxel_render,
        };

        render
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;

        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn render(&mut self) {
        self.update_view_projection();

        if self.world.read().unwrap().update_window > 0 {
            self.update_chunks();
        }

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
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
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

        render_pass.set_pipeline(&self.voxel_render.pipeline);
        render_pass.set_bind_group(0, &self.view_projection_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.voxel_render.instance_buffer.slice(..));

        render_pass.draw(0..36, 0..self.voxel_render.instance_count);

        drop(render_pass);

        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();

        surface_texture.present();
    }

    fn update_chunks(&mut self) {
        let voxel_instances = self.read_chunks();
        let voxel_instance_count = voxel_instances.len() as u32;

        if voxel_instance_count > 0 {
            let voxel_instance_buffer =
                self.device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Voxel Instance Buffer"),
                        contents: bytemuck::cast_slice(voxel_instances.as_slice()),
                        usage: wgpu::BufferUsages::VERTEX,
                    });

            self.voxel_render.instance_buffer = voxel_instance_buffer;
            self.voxel_render.instance_count = voxel_instance_count;
        }
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::RedrawRequested => {
                self.render();
                self.window.request_redraw();
            }
            WindowEvent::Resized(size) => {
                self.resize(*size);
            }
            _ => (),
        }
    }

    fn read_chunks(&self) -> Vec<VoxelInstance> {
        let mut voxel_instances: Vec<VoxelInstance> = Vec::new();

        for (chunk_id, chunk) in self.chunks.iter().enumerate() {
            let chunk_id = chunk_id as u32;
            let chunk = chunk.read().unwrap();

            for block_id in 0..CHUNK_VOLUME {
                let palette_id = chunk.blocks[block_id as usize];
                let kind = chunk.palette[palette_id as usize];

                if kind != BlockKind::Air {
                    let grid_position = Simulation::get_grid_position(chunk_id, block_id);
                    let block = &BLOCKS[kind as usize];

                    let voxel_instance = self.create_voxel_instance(grid_position, block);
                    voxel_instances.push(voxel_instance);
                }
            }
        }

        voxel_instances
    }

    fn update_view_projection(&mut self) {
        let view_projection_matrix = Render::create_view_projection_matrix(self.agent.clone());

        self.queue.write_buffer(
            &self.view_projection_buffer,
            0,
            bytemuck::cast_slice(&view_projection_matrix),
        );
    }

    fn create_voxel_instance(&self, grid_position: IVec3, block: &Block) -> VoxelInstance {
        VoxelInstance {
            position: grid_position.as_vec3().into(),
            color: [
                block.color.0 as f32,
                block.color.1 as f32,
                block.color.2 as f32,
                block.color.3 as f32,
            ],
        }
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

    // fn sort_instances_by_depth(camera_position: Vec3, instances: &mut Vec<VoxelInstance>) {
    //     instances.sort_by(|a, b| {
    //         let dist_a = ((a.position[0] - camera_position.x as f32).powi(2)
    //             + (a.position[1] - camera_position.y as f32).powi(2)
    //             + (a.position[2] - camera_position.z as f32).powi(2))
    //         .sqrt();

    //         let dist_b = ((b.position[0] - camera_position.x as f32).powi(2)
    //             + (b.position[1] - camera_position.y as f32).powi(2)
    //             + (b.position[2] - camera_position.z as f32).powi(2))
    //         .sqrt();

    //         dist_b.partial_cmp(&dist_a).unwrap()
    //     });
    // }

    fn create_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        shader: &wgpu::ShaderModule,
        pipeline_layout: wgpu::PipelineLayout,
        instance_layout: wgpu::VertexBufferLayout<'_>,
    ) -> wgpu::RenderPipeline {
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Voxel Render Pipeline"),
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
