use crate::interface::{
    gpu::gpu_mesh::GpuMesh, renderer::world_renderer::block_renderer::block_vertex_data::BlockVertexData,
};

pub struct BlockModel {
    pub vertex_vec: Vec<BlockVertexData>,
    pub index_vec: Vec<u32>,
}

impl BlockModel {
    pub fn to_gpu_mesh(block_model: &Self, device: &wgpu::Device) -> GpuMesh {
        assert!(
            !block_model.vertex_vec.is_empty(),
            "Vertex buffer is empty!"
        );
        assert!(!block_model.index_vec.is_empty(), "Index buffer is empty!");

        let vertex_buffer = wgpu::util::DeviceExt::create_buffer_init(
            device,
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&block_model.vertex_vec),
                usage: wgpu::BufferUsages::VERTEX,
            },
        );

        let index_buffer = wgpu::util::DeviceExt::create_buffer_init(
            device,
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&block_model.index_vec),
                usage: wgpu::BufferUsages::INDEX,
            },
        );

        let index_count = block_model.index_vec.len() as u32;

        let material_id = 0;

        GpuMesh {
            version: 0,
            vertex_buffer,
            index_buffer,
            index_count,
            material_id,
        }
    }
}
