use crate::{
    consts::{ASPECT_RATIO, CHUNK_SIZE, FAR_PLANE, FOV, NEAR_PLANE},
    include_shader_src,
    simulation::{block::BlockType, state::{Judge, World}},
};
use bytemuck::{Pod, Zeroable};
use cgmath::{perspective, Deg, Matrix4, Point3, Vector3};
use rand::Rng;
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

pub struct Voxel {
    shader: wgpu::ShaderModule,
    instances: Vec<VoxelInstance>,
    instances_count: u32,
    instance_buffer: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct VoxelInstance {
    position: [f32; 3],
    color: [f32; 4],
}

pub struct Render {
    window: Arc<Window>,
    judge: Arc<RwLock<Judge>>,
    world: Arc<RwLock<World>>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
    surface_config: wgpu::SurfaceConfiguration,
    view_projection_bind_group: wgpu::BindGroup,
    voxel: Voxel,
}

impl Render {
    pub async fn new(window: Arc<Window>, judge: Arc<RwLock<Judge>>, world: Arc<RwLock<World>>) -> Render {
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
            view_formats: vec![surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: size.width,
            height: size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };

        surface.configure(&device, &surface_config);

        let view_projection_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("View Projection Buffer"),
            size: std::mem::size_of::<[[f32; 4]; 4]>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let view_projection_matrix = create_view_projection_matrix(judge.clone());

        queue.write_buffer(
            &view_projection_buffer,
            0,
            bytemuck::cast_slice(&view_projection_matrix),
        );

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

        let mut voxel_instances = read_world(world.clone());
        sort_voxels_by_depth(judge.read().unwrap().position, &mut voxel_instances);
        let voxel_instances_count = voxel_instances.len() as u32;
        let voxel_instance_buffer = create_instance_buffer(&device, voxel_instances.as_slice());

        let voxel_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Voxel Pipeline Layout"),
                bind_group_layouts: &[&uniform_bind_group_layout],
                push_constant_ranges: &[],
            });

        let voxel_pipeline = create_pipeline(
            &device,
            &surface_config,
            &voxel_shader,
            voxel_pipeline_layout,
            VOXEL_INSTANCE_LAYOUT,
        );

        let voxel = Voxel {
            shader: voxel_shader,
            instances: voxel_instances,
            instances_count: voxel_instances_count,
            instance_buffer: voxel_instance_buffer,
            pipeline: voxel_pipeline,
        };

        let render = Render {
            window,
            judge,
            world,
            device,
            queue,
            size,
            surface,
            surface_format,
            surface_config,
            view_projection_bind_group,
            voxel,
        };

        render
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;

        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn render(&mut self) {
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

        let mut encoder = self.device.create_command_encoder(&Default::default());

        let depth_texture_view = create_depth_texture(&self.device, &self.surface_config);

        let mut renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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

        renderpass.set_pipeline(&self.voxel.pipeline);
        renderpass.set_bind_group(0, &self.view_projection_bind_group, &[]);
        renderpass.set_vertex_buffer(0, self.voxel.instance_buffer.slice(..));
        renderpass.draw(0..36, 0..self.voxel.instances_count);

        drop(renderpass);

        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        surface_texture.present();
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
}

fn read_world(world: Arc<RwLock<World>>) -> Vec<VoxelInstance> {
    let world = world.read().unwrap();

    let mut instances: Vec<VoxelInstance> = Vec::new();

    for chunk in world.chunks.iter() {
        for block in chunk.blocks.iter() {
            match block.block_type {
                BlockType::Solid => {
                    let instance = VoxelInstance {
                        position: [
                            (chunk.position.x * CHUNK_SIZE as i64 + block.position.x) as f32,
                            (chunk.position.y * CHUNK_SIZE as i64 + block.position.y) as f32,
                            (chunk.position.z * CHUNK_SIZE as i64 + block.position.z) as f32,
                        ],
                        color: [
                            block.color.x as f32,
                            block.color.y as f32,
                            block.color.z as f32,
                            block.color.w as f32,
                        ],
                    };
        
                    instances.push(instance);
                }
                BlockType::Empty => ()
            }
        }
    }

    instances
}

fn create_instance_buffer(device: &wgpu::Device, instances: &[VoxelInstance]) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Voxel Instance Buffer"),
        contents: bytemuck::cast_slice(instances),
        usage: wgpu::BufferUsages::VERTEX,
    })
}

fn create_depth_texture(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> wgpu::TextureView {
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
        format: wgpu::TextureFormat::Depth24Plus,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });

    depth_texture.create_view(&wgpu::TextureViewDescriptor::default())
}

fn sort_voxels_by_depth(camera_position: Vector3<f64>, voxels: &mut Vec<VoxelInstance>) {
    voxels.sort_by(|a, b| {
        let dist_a = (a.position[2] - camera_position.z as f32).abs();
        let dist_b = (b.position[2] - camera_position.z as f32).abs();

        dist_b.partial_cmp(&dist_a).unwrap()
    });
}

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
                    color: wgpu::BlendComponent::OVER,
                    alpha: wgpu::BlendComponent::OVER,
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
            format: wgpu::TextureFormat::Depth24Plus,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less, // Closer fragments replace further ones
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    })
}

fn create_view_projection_matrix(judge: Arc<RwLock<Judge>>) -> [[f32; 4]; 4] {
    let judge = judge.read().unwrap();

    let proj = perspective(Deg(FOV), ASPECT_RATIO, NEAR_PLANE, FAR_PLANE);

    let eye = Point3::new(judge.position.x as f32, judge.position.y as f32, judge.position.z as f32);
    let target = Point3::new(0.0, 0.0, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);

    let view = Matrix4::look_at_rh(eye, target, up);

    let view_proj = proj * view;

    view_proj.into()
}
