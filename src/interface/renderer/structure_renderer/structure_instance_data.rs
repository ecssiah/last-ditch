#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct StructureInstanceData {
    pub world_position: [f32; 3],
    pub rotation_xy: f32,
}

impl StructureInstanceData {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        3 => Float32x3,
        4 => Float32,
    ];

    pub fn new(world_position: [f32; 3], rotation_xy: f32) -> Self {
        Self {
            world_position,
            rotation_xy,
        }
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}
