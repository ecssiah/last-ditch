
#[derive(Debug)]
pub struct Mesh {
    pub last_update: Tick,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}

impl Mesh {
    pub fn new(
        device: &wgpu::Device,
        creation_tick: Tick,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    ) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Interface Chunk Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Interface Chunk Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            last_update: creation_tick,
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as u32,
        }
    }
}