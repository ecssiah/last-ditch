use crate::{
    include_assets,
    interface::{
        camera::Camera,
        debug::{debug_vertex_data::DebugVertexData, DebugChannel, DebugVisibility},
        gpu::gpu_context::GPUContext,
    },
    simulation::{constants::CELL_RADIUS, viewer::View},
};
use ultraviolet::Vec3;

pub struct DebugRender {
    pub visible: bool,
    pub debug_visibility: DebugVisibility,
    pub channel_vertex_vec_array: [Vec<DebugVertexData>; DebugChannel::ALL.len()],
    pub render_pipeline: wgpu::RenderPipeline,
    pub camera_bind_group: wgpu::BindGroup,
    pub vertex_buffer: wgpu::Buffer,
    pub vertex_capacity: usize,
    pub vertex_vec: Vec<DebugVertexData>,
}

impl DebugRender {
    pub fn new(gpu_context: &GPUContext, camera: &Camera) -> Self {
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
                        buffers: &[DebugVertexData::desc()],
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

        let initial_capacity = 64;

        let vertex_buffer = gpu_context.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Debug Lines Vertex Buffer"),
            size: (initial_capacity * std::mem::size_of::<DebugVertexData>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let visible = true;
        let debug_visibility = DebugVisibility::SECTOR_BORDERS;

        let channel_vertex_vec_array: [Vec<DebugVertexData>; DebugChannel::ALL.len()] =
            std::array::from_fn(|_| Vec::new());

        let vertex_vec = Vec::new();

        Self {
            visible,
            debug_visibility,
            channel_vertex_vec_array,
            render_pipeline,
            camera_bind_group: camera.uniform_bind_group.clone(),
            vertex_buffer,
            vertex_capacity: initial_capacity,
            vertex_vec,
        }
    }

    #[inline]
    pub fn clear_channel_vertex_vec(
        channel_vertex_vec_array: &mut [Vec<DebugVertexData>; DebugChannel::ALL.len()],
    ) {
        for vertex_vec in channel_vertex_vec_array {
            vertex_vec.clear();
        }
    }

    pub fn add_line(
        debug_channel: DebugChannel,
        position1: Vec3,
        position2: Vec3,
        color: [f32; 3],
        channel_vertex_vec_array: &mut [Vec<DebugVertexData>; DebugChannel::ALL.len()],
    ) {
        let vertex_vec = &mut channel_vertex_vec_array[DebugChannel::index(debug_channel)];

        vertex_vec.push(DebugVertexData {
            position: position1.into(),
            color,
        });

        vertex_vec.push(DebugVertexData {
            position: position2.into(),
            color,
        });
    }

    pub fn add_ray(
        debug_channel: DebugChannel,
        origin: Vec3,
        direction: Vec3,
        length: f32,
        color: [f32; 3],
        channel_vertex_vec_array: &mut [Vec<DebugVertexData>; DebugChannel::ALL.len()],
    ) {
        if direction.mag_sq() > 0.0 {
            Self::add_line(
                debug_channel,
                origin,
                origin + direction.normalized() * length,
                color,
                channel_vertex_vec_array,
            );
        }
    }

    pub fn add_axes(
        debug_channel: DebugChannel,
        origin: Vec3,
        scale: f32,
        channel_vertex_vec_array: &mut [Vec<DebugVertexData>; DebugChannel::ALL.len()],
    ) {
        Self::add_line(
            debug_channel,
            origin,
            origin + Vec3::unit_x() * scale,
            [1.0, 0.1, 0.1],
            channel_vertex_vec_array,
        );

        Self::add_line(
            debug_channel,
            origin,
            origin + Vec3::unit_y() * scale,
            [0.1, 1.0, 0.1],
            channel_vertex_vec_array,
        );

        Self::add_line(
            debug_channel,
            origin,
            origin + Vec3::unit_z() * scale,
            [0.1, 0.1, 1.0],
            channel_vertex_vec_array,
        );
    }

    pub fn add_box(
        debug_channel: DebugChannel,
        min: Vec3,
        max: Vec3,
        color: [f32; 3],
        channel_vertex_vec_array: &mut [Vec<DebugVertexData>; DebugChannel::ALL.len()],
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
                channel_vertex_vec_array,
            );
        }
    }

    pub fn apply_debug_view(view: &View, debug_render: &mut DebugRender) {
        if !view.admin_view.debug_active {
            return;
        }

        if debug_render
            .debug_visibility
            .contains(DebugVisibility::CHANNEL1)
        {}

        if debug_render
            .debug_visibility
            .contains(DebugVisibility::SECTOR_BORDERS)
        {
            let radius = view.world_view.grid.world_radius_in_sectors as i32;
            let sector_size_in_cells: f32 = view.world_view.grid.sector_size_in_meters;

            let half_span = (radius as f32 + CELL_RADIUS) * sector_size_in_cells;
            let min = Vec3::broadcast(-half_span);
            let max = Vec3::broadcast(half_span);

            let mut bounds: Vec<f32> = Vec::with_capacity(2 * radius as usize + 2);

            for k in -(radius + 1)..=radius {
                bounds.push((k as f32 + CELL_RADIUS) * sector_size_in_cells);
            }

            for &y in &bounds {
                for &z in &bounds {
                    Self::add_line(
                        DebugChannel::SectorBorders,
                        Vec3::new(min.x, y, z),
                        Vec3::new(max.x, y, z),
                        [1.0, 0.0, 0.0],
                        &mut debug_render.channel_vertex_vec_array,
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
                        &mut debug_render.channel_vertex_vec_array,
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
                        &mut debug_render.channel_vertex_vec_array,
                    );
                }
            }
        }
    }

    pub fn render(
        surface_texture_view: &wgpu::TextureView,
        depth_texture_view: &wgpu::TextureView,
        gpu_context: &GPUContext,
        debug_render: &mut DebugRender,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        if !debug_render.visible {
            return;
        }

        debug_render.vertex_vec.clear();

        for (index, vertex_vec) in debug_render.channel_vertex_vec_array.iter().enumerate() {
            let mask = DebugChannel::mask(DebugChannel::ALL[index]);

            if debug_render.debug_visibility.contains(mask) {
                debug_render.vertex_vec.extend_from_slice(&vertex_vec);
            }
        }

        if debug_render.vertex_vec.is_empty() {
            return;
        }

        if debug_render.vertex_capacity < debug_render.vertex_vec.len() {
            debug_render.vertex_capacity = debug_render.vertex_vec.len().next_power_of_two();

            debug_render.vertex_buffer =
                gpu_context.device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("Debug Render Vertex Buffer"),
                    size: (debug_render.vertex_capacity * std::mem::size_of::<DebugVertexData>())
                        as u64,
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
        }

        gpu_context.queue.write_buffer(
            &debug_render.vertex_buffer,
            0,
            bytemuck::cast_slice(&debug_render.vertex_vec),
        );

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

        render_pass.set_pipeline(&debug_render.render_pipeline);
        render_pass.set_bind_group(0, &debug_render.camera_bind_group, &[]);
        render_pass.set_vertex_buffer(0, debug_render.vertex_buffer.slice(..));
        render_pass.draw(0..(debug_render.vertex_vec.len() as u32), 0..1);

        Self::clear_channel_vertex_vec(&mut debug_render.channel_vertex_vec_array);
    }
}
