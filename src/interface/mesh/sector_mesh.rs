use crate::{
    interface::{gpu::gpu_mesh::GpuMesh, mesh::block_vertex::BlockVertex},
    simulation::state::world::sector,
};

pub struct SectorMesh {
    pub sector_id: sector::ID,
    pub version: u64,
    pub vertex_vec: Vec<BlockVertex>,
    pub index_vec: Vec<u32>,
}

impl SectorMesh {
    pub fn to_gpu_mesh(sector_mesh: &SectorMesh, device: &wgpu::Device) -> GpuMesh {
        assert!(
            !sector_mesh.vertex_vec.is_empty(),
            "Vertex buffer is empty!"
        );

        assert!(!sector_mesh.index_vec.is_empty(), "Index buffer is empty!");

        let vertex_buffer = wgpu::util::DeviceExt::create_buffer_init(
            device,
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&sector_mesh.vertex_vec),
                usage: wgpu::BufferUsages::VERTEX,
            },
        );

        let index_buffer = wgpu::util::DeviceExt::create_buffer_init(
            device,
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&sector_mesh.index_vec),
                usage: wgpu::BufferUsages::INDEX,
            },
        );

        let index_count = sector_mesh.index_vec.len() as u32;

        let material_id = 0;

        GpuMesh {
            version: sector_mesh.version,
            vertex_buffer,
            index_buffer,
            index_count,
            material_id,
        }
    }
}
