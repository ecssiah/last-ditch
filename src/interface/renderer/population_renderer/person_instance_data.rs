#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PersonInstanceData {
    pub world_position: [f32; 3],
    pub scale: f32,
    pub rotation_xy: f32,
    pub _padding: [f32; 3],
}

impl PersonInstanceData {
    const ATTRIBS: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![
        3 => Float32x3,
        4 => Float32,
        5 => Float32,
    ];

    pub fn new(world_position: [f32; 3], scale: f32, rotation_xy: f32) -> Self {
        let padding = [0.0, 0.0, 0.0];

        Self {
            world_position,
            scale,
            rotation_xy,
            _padding: padding,
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
