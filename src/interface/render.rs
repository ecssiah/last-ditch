use crate::{
    consts::{ASPECT_RATIO, FAR_PLANE, FOV, NEAR_PLANE},
    include_shader_src,
    simulation::state::World,
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
    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32],
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
    _padding: f32,
}

pub struct Render {
    window: Arc<Window>,
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
    pub async fn new(window: Arc<Window>, world: Arc<RwLock<World>>) -> Render {
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

        let view_projection_matrix = create_view_projection_matrix();
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

        let voxel_instances = generate_world();
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
            depth_stencil_attachment: None,
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

fn generate_world() -> Vec<VoxelInstance> {
    let mut instances = Vec::new();
    let size = 10;

    let mut rng = rand::thread_rng();

    for x in 0..size {
        for y in 0..size {
            for z in 0..size {
                let roll: f32 = rng.gen();

                if roll < 0.25 {
                    instances.push(VoxelInstance {
                        position: [x as f32, y as f32, z as f32],
                        _padding: 0.0,
                    });
                }
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
                blend: Some(wgpu::BlendState::REPLACE),
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
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    })
}

fn create_view_projection_matrix() -> [[f32; 4]; 4] {
    let proj = perspective(Deg(FOV), ASPECT_RATIO, NEAR_PLANE, FAR_PLANE);

    let eye = Point3::new(20.0, 20.0, 20.0);
    let target = Point3::new(0.0, 0.0, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);

    let view = Matrix4::look_at_rh(eye, target, up);

    let view_proj = proj * view;

    view_proj.into()
}
