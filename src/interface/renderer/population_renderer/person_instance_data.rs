#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PersonInstanceData {
    pub world_position: [f32; 3],
    pub scale: f32,
    pub rotation_xy: f32,
    pub atlas_index: u32,
    pub layer_index: u32,
}

impl PersonInstanceData {
    const ATTRIBS: [wgpu::VertexAttribute; 5] = wgpu::vertex_attr_array![
        3 => Float32x3,
        4 => Float32,
        5 => Float32,
        6 => Uint32,
        7 => Uint32,
    ];

    pub fn new(
        world_position: [f32; 3],
        scale: f32,
        rotation_xy: f32,
        atlas_index: u32,
        layer_index: u32,
    ) -> Self {
        Self {
            world_position,
            scale,
            rotation_xy,
            atlas_index,
            layer_index,
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
