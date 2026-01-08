#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BlockInstanceData {
    pub world_position: [f32; 3],
    pub rotation_xy: f32,
    pub layer_index: u32,
}

impl BlockInstanceData {
    const ATTRIBUTES: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![
        3 => Float32x3,
        4 => Float32,
        5 => Uint32,
    ];

    pub fn new(world_position: [f32; 3], rotation_xy: f32, layer_index: u32) -> Self {
        Self {
            world_position,
            rotation_xy,
            layer_index,
        }
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBUTES,
        }
    }
}
