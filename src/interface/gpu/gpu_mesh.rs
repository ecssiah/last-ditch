pub struct GpuMesh {
    pub version: u64,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
    pub material_id: u32,
}
