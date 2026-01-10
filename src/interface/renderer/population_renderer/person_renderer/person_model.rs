use crate::interface::{
    gpu::gpu_mesh::GpuMesh,
    renderer::population_renderer::person_renderer::person_vertex_data::PersonVertexData,
};

pub struct PersonModel {
    pub vertex_vec: Vec<PersonVertexData>,
    pub index_vec: Vec<u32>,
}

impl PersonModel {
    pub fn to_gpu_mesh(person_model: &Self, device: &wgpu::Device) -> GpuMesh {
        assert!(
            !person_model.vertex_vec.is_empty(),
            "Vertex buffer is empty!"
        );
        assert!(!person_model.index_vec.is_empty(), "Index buffer is empty!");

        let vertex_buffer = wgpu::util::DeviceExt::create_buffer_init(
            device,
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&person_model.vertex_vec),
                usage: wgpu::BufferUsages::VERTEX,
            },
        );

        let index_buffer = wgpu::util::DeviceExt::create_buffer_init(
            device,
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&person_model.index_vec),
                usage: wgpu::BufferUsages::INDEX,
            },
        );

        let index_count = person_model.index_vec.len() as u32;

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
