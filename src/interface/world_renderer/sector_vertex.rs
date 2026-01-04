use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct SectorVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub tile_indices: [u32; 2],
}

impl SectorVertex {
    pub const ATTRS: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x3,
        2 => Float32x2,
        3 => Uint32x2,
    ];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRS,
        }
    }
}
