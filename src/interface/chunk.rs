pub struct Chunk {
    pub last_render: u64,
    pub instance_count: u32,
    pub instance_buffer: wgpu::Buffer,
}
