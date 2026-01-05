pub mod debug_channel;
pub mod debug_vertex;

use crate::{
    include_assets,
    interface::{
        camera::Camera,
        gpu::gpu_context::GPUContext,
        renderer::debug_renderer::{debug_channel::DebugChannel, debug_vertex::DebugVertex},
    },
    simulation::{constants::*, supervisor::viewer::view::View},
};
use std::collections::{HashMap, HashSet};
use tracing::instrument;
use ultraviolet::Vec3;

pub struct DebugRenderer {
    pub debug_active: bool,
    pub camera_bind_group: wgpu::BindGroup,
    pub vertex_capacity: usize,
    pub vertex_vec: Vec<DebugVertex>,
    pub vertex_buffer: wgpu::Buffer,
    pub channel_set: HashSet<DebugChannel>,
    pub channel_vertex_map: HashMap<DebugChannel, Vec<DebugVertex>>,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl DebugRenderer {
    pub fn new(gpu_context: &GPUContext, camera: &Camera) -> Self {
        let debug_active = false;

        let vert_shader_module =
            gpu_context
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Debug Vertex Shader"),
                    source: wgpu::ShaderSource::Wgsl(
                        include_assets!("shaders/debug.vert.wgsl").into(),
                    ),
                });

        let frag_shader_module =
            gpu_context
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Debug Fragment Shader"),
                    source: wgpu::ShaderSource::Wgsl(
                        include_assets!("shaders/debug.frag.wgsl").into(),
                    ),
                });

        let vertex_vec = Vec::new();
        let vertex_capacity = 64;

        let vertex_buffer = gpu_context.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Debug Vertex Buffer"),
            size: (vertex_capacity * std::mem::size_of::<DebugVertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let channel_set = HashSet::from([DebugChannel::SectorBorders, DebugChannel::Custom]);

        let channel_vertex_map: HashMap<_, _> = DebugChannel::ALL
            .into_iter()
            .map(|debug_channel| {
                let key = debug_channel;
                let value = Vec::new();
                (key, value)
            })
            .collect();

        let pipeline_layout =
            gpu_context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Debug Render Pipeline"),
                    bind_group_layouts: &[&camera.uniform_bind_group_layout],
                    push_constant_ranges: &[],
                });

        let render_pipeline =
            gpu_context
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Debug Render Pipeline"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &vert_shader_module,
                        entry_point: Some("main"),
                        buffers: &[DebugVertex::desc()],
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &frag_shader_module,
                        entry_point: Some("main"),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: gpu_context.surface_config.format,
                            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::LineList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: None,
                        unclipped_depth: false,
                        polygon_mode: wgpu::PolygonMode::Fill,
                        conservative: false,
                    },
                    depth_stencil: Some(wgpu::DepthStencilState {
                        format: wgpu::TextureFormat::Depth32Float,
                        depth_write_enabled: false,
                        depth_compare: wgpu::CompareFunction::LessEqual,
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState {
                            constant: -2,
                            slope_scale: -1.0,
                            clamp: 0.0,
                        },
                    }),
                    multisample: wgpu::MultisampleState::default(),
                    multiview: None,
                    cache: None,
                });

        Self {
            debug_active,
            camera_bind_group: camera.uniform_bind_group.clone(),
            vertex_vec,
            vertex_capacity,
            vertex_buffer,
            channel_set,
            channel_vertex_map,
            render_pipeline,
        }
    }

    pub fn toggle_debug_active(debug_renderer: &mut Self) {
        debug_renderer.debug_active = !debug_renderer.debug_active;
    }

    pub fn activate_channel(debug_channel: DebugChannel, debug_renderer: &mut Self) {
        debug_renderer.channel_set.insert(debug_channel);
    }

    pub fn deactivate_channel(debug_channel: DebugChannel, debug_renderer: &mut Self) {
        debug_renderer.channel_set.remove(&debug_channel);
    }

    pub fn add_line(
        debug_channel: DebugChannel,
        position1: Vec3,
        position2: Vec3,
        color: [f32; 3],
        channel_vertex_map: &mut HashMap<DebugChannel, Vec<DebugVertex>>,
    ) {
        if let Some(vertex_vec) = channel_vertex_map.get_mut(&debug_channel) {
            vertex_vec.push(DebugVertex {
                position: position1.into(),
                color,
            });

            vertex_vec.push(DebugVertex {
                position: position2.into(),
                color,
            });
        }
    }

    pub fn add_ray(
        debug_channel: DebugChannel,
        origin: Vec3,
        direction: Vec3,
        length: f32,
        color: [f32; 3],
        channel_vertex_map: &mut HashMap<DebugChannel, Vec<DebugVertex>>,
    ) {
        if direction.mag_sq() > 0.0 {
            Self::add_line(
                debug_channel,
                origin,
                origin + direction.normalized() * length,
                color,
                channel_vertex_map,
            );
        }
    }

    pub fn add_axes(
        debug_channel: DebugChannel,
        origin: Vec3,
        scale: f32,
        channel_vertex_map: &mut HashMap<DebugChannel, Vec<DebugVertex>>,
    ) {
        Self::add_line(
            debug_channel,
            origin,
            origin + Vec3::unit_x() * scale,
            [1.0, 0.1, 0.1],
            channel_vertex_map,
        );

        Self::add_line(
            debug_channel,
            origin,
            origin + Vec3::unit_y() * scale,
            [0.1, 1.0, 0.1],
            channel_vertex_map,
        );

        Self::add_line(
            debug_channel,
            origin,
            origin + Vec3::unit_z() * scale,
            [0.1, 0.1, 1.0],
            channel_vertex_map,
        );
    }

    pub fn add_box(
        debug_channel: DebugChannel,
        min: Vec3,
        max: Vec3,
        color: [f32; 3],
        channel_vertex_map: &mut HashMap<DebugChannel, Vec<DebugVertex>>,
    ) {
        let (x0, y0, z0) = (min.x, min.y, min.z);
        let (x1, y1, z1) = (max.x, max.y, max.z);

        let edge_array = [
            (Vec3::new(x0, y0, z0), Vec3::new(x1, y0, z0)),
            (Vec3::new(x1, y0, z0), Vec3::new(x1, y1, z0)),
            (Vec3::new(x1, y1, z0), Vec3::new(x0, y1, z0)),
            (Vec3::new(x0, y1, z0), Vec3::new(x0, y0, z0)),
            (Vec3::new(x0, y0, z1), Vec3::new(x1, y0, z1)),
            (Vec3::new(x1, y0, z1), Vec3::new(x1, y1, z1)),
            (Vec3::new(x1, y1, z1), Vec3::new(x0, y1, z1)),
            (Vec3::new(x0, y1, z1), Vec3::new(x0, y0, z1)),
            (Vec3::new(x0, y0, z0), Vec3::new(x0, y0, z1)),
            (Vec3::new(x1, y0, z0), Vec3::new(x1, y0, z1)),
            (Vec3::new(x1, y1, z0), Vec3::new(x1, y1, z1)),
            (Vec3::new(x0, y1, z0), Vec3::new(x0, y1, z1)),
        ];

        for (position1, position2) in edge_array {
            Self::add_line(
                debug_channel,
                position1,
                position2,
                color,
                channel_vertex_map,
            );
        }
    }

    #[instrument(skip_all)]
    pub fn apply_debug_view(gpu_context: &GPUContext, _view: &View, debug_renderer: &mut Self) {
        if !debug_renderer.debug_active {
            return;
        }

        if debug_renderer
            .channel_set
            .contains(&DebugChannel::SectorBorders)
        {
            let world_radius_in_sectors = WORLD_RADIUS_IN_SECTORS as i32;
            let sector_size_in_cells: f32 = SECTOR_SIZE_IN_CELLS as f32;

            let half_span =
                (world_radius_in_sectors as f32 + CELL_RADIUS_IN_METERS) * sector_size_in_cells;
            let min = Vec3::broadcast(-half_span);
            let max = Vec3::broadcast(half_span);

            let mut bounds: Vec<f32> = Vec::with_capacity(2 * world_radius_in_sectors as usize + 2);

            for k in -(world_radius_in_sectors + 1)..=world_radius_in_sectors {
                bounds.push((k as f32 + CELL_RADIUS_IN_METERS) * sector_size_in_cells);
            }

            for &y in &bounds {
                for &z in &bounds {
                    Self::add_line(
                        DebugChannel::SectorBorders,
                        Vec3::new(min.x, y, z),
                        Vec3::new(max.x, y, z),
                        [1.0, 0.0, 0.0],
                        &mut debug_renderer.channel_vertex_map,
                    );
                }
            }

            for &x in &bounds {
                for &z in &bounds {
                    Self::add_line(
                        DebugChannel::SectorBorders,
                        Vec3::new(x, min.y, z),
                        Vec3::new(x, max.y, z),
                        [0.0, 1.0, 0.0],
                        &mut debug_renderer.channel_vertex_map,
                    );
                }
            }

            for &x in &bounds {
                for &y in &bounds {
                    Self::add_line(
                        DebugChannel::SectorBorders,
                        Vec3::new(x, y, min.z),
                        Vec3::new(x, y, max.z),
                        [0.0, 0.0, 1.0],
                        &mut debug_renderer.channel_vertex_map,
                    );
                }
            }
        }

        if debug_renderer.channel_set.contains(&DebugChannel::Custom) {}

        debug_renderer.vertex_vec.clear();

        for debug_channel in &debug_renderer.channel_set {
            if let Some(vertex_vec) = debug_renderer.channel_vertex_map.get_mut(&debug_channel) {
                debug_renderer.vertex_vec.append(vertex_vec);
            }
        }

        if debug_renderer.vertex_capacity < debug_renderer.vertex_vec.len() {
            debug_renderer.vertex_capacity = debug_renderer.vertex_vec.len().next_power_of_two();

            debug_renderer.vertex_buffer =
                gpu_context.device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("Debug Render Vertex Buffer"),
                    size: (debug_renderer.vertex_capacity * std::mem::size_of::<DebugVertex>())
                        as u64,
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
        }

        gpu_context.queue.write_buffer(
            &debug_renderer.vertex_buffer,
            0,
            bytemuck::cast_slice(&debug_renderer.vertex_vec),
        );
    }

    #[instrument(skip_all)]
    pub fn render(
        surface_texture_view: &wgpu::TextureView,
        depth_texture_view: &wgpu::TextureView,
        debug_renderer: &mut Self,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        if !debug_renderer.debug_active || debug_renderer.vertex_vec.is_empty() {
            return;
        }

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Debug Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: surface_texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth_texture_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&debug_renderer.render_pipeline);
        render_pass.set_bind_group(0, &debug_renderer.camera_bind_group, &[]);
        render_pass.set_vertex_buffer(0, debug_renderer.vertex_buffer.slice(..));
        render_pass.draw(0..(debug_renderer.vertex_vec.len() as u32), 0..1);
    }
}
