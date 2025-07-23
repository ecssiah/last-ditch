use crate::interface::mesh_render::data::VertexData;
use wgpu::util::DeviceExt;

#[derive(Clone, Debug)]
pub struct MeshData {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}

impl MeshData {
    pub fn new(
        device: &wgpu::Device,
        textured_vertex_vec: Vec<obj::TexturedVertex>,
        index_vec: Vec<u32>,
    ) -> Self {
        let vertex_data_vec: Vec<VertexData> = textured_vertex_vec
            .iter()
            .map(|vertex| VertexData {
                position: vertex.position,
                normal: vertex.normal,
                uv: [vertex.texture[0], vertex.texture[1]],
            })
            .collect();

        assert!(!vertex_data_vec.is_empty(), "Vertex buffer is empty!");
        assert!(!index_vec.is_empty(), "Index buffer is empty!");

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&vertex_data_vec),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
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
