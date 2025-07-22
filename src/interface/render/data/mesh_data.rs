use crate::interface::render::data::VertexData;
use wgpu::util::DeviceExt;

#[derive(Debug)]
pub struct MeshData {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}

impl MeshData {
    pub fn new(device: &wgpu::Device, vertex_vec: Vec<VertexData>, index_vec: Vec<u32>) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Interface Chunk Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_vec),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Interface Chunk Index Buffer"),
            contents: bytemuck::cast_slice(&index_vec),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            vertex_buffer,
            index_buffer,
            index_count: index_vec.len() as u32,
        }
    }
}
