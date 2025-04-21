use wgpu::util::DeviceExt;

use crate::interface::render::gpu_fog::GPUFog;

pub struct FogRender {
    pub gpu_fog: GPUFog,
    pub buffer: wgpu::Buffer,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
    pub uniform_bind_group: wgpu::BindGroup,
}

impl FogRender {
    pub fn new(device: &wgpu::Device) -> FogRender {
        let gpu_fog = GPUFog {
            color: [0.5, 0.5, 0.5],
            _padding0: 0.0,
            start: 52.0,
            end: 120.0,
            _padding1: [0.0, 0.0],
        };

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Fog Uniform Buffer"),
            contents: bytemuck::bytes_of(&gpu_fog),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Fog Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("Fog Bind Group"),
        });

        let fog_render = FogRender {
            gpu_fog,
            buffer,
            uniform_bind_group_layout,
            uniform_bind_group,
        };

        fog_render
    }
}
