use crate::interface::render::fog_uniform_data::FogUniformData;
use wgpu::util::DeviceExt;

pub struct Fog {
    pub fog_uniform_data: FogUniformData,
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
    pub uniform_bind_group: wgpu::BindGroup,
}

impl Fog {
    pub fn new(device: &wgpu::Device) -> Fog {
        let fog_uniform_data = FogUniformData {
            color: [0.5, 0.5, 0.5],
            _padding0: 0.0,
            start: 60.0,
            end: 180.0,
            _padding1: [0.0, 0.0],
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Fog Uniform Buffer"),
            contents: bytemuck::bytes_of(&fog_uniform_data),
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
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("Fog Bind Group"),
        });

        let fog = Fog {
            fog_uniform_data,
            uniform_buffer,
            uniform_bind_group_layout,
            uniform_bind_group,
        };

        fog
    }
}
