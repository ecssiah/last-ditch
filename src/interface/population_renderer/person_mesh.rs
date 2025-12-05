use crate::interface::{gpu::gpu_mesh::GpuMesh, population_renderer::person_vertex::PersonVertex};

pub struct PersonMesh {
    pub vertex_vec: Vec<PersonVertex>,
    pub index_vec: Vec<u32>,
}

impl PersonMesh {
    pub fn to_gpu_mesh(person_mesh: &Self, device: &wgpu::Device) -> GpuMesh {
        assert!(
            !person_mesh.vertex_vec.is_empty(),
            "Vertex buffer is empty!"
        );
        assert!(!person_mesh.index_vec.is_empty(), "Index buffer is empty!");

        let vertex_buffer = wgpu::util::DeviceExt::create_buffer_init(
            device,
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&person_mesh.vertex_vec),
                usage: wgpu::BufferUsages::VERTEX,
            },
        );

        let index_buffer = wgpu::util::DeviceExt::create_buffer_init(
            device,
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&person_mesh.index_vec),
                usage: wgpu::BufferUsages::INDEX,
            },
        );

        let index_count = person_mesh.index_vec.len() as u32;

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
