use crate::simulation::{chunk::ChunkID, WORLD_VOLUME};
use bytemuck::{Pod, Zeroable};
use std::collections::HashMap;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct ChunkVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 4],
    pub ao: u32,
}

impl ChunkVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x3,
        2 => Float32x4,
        3 => Uint32,
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ChunkVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ChunkMesh {
    pub last_render: u64,
    pub vertices: Vec<ChunkVertex>,
    pub indices: Vec<u32>,
}

#[derive(Debug, Default)]
pub struct ChunkMeshCache {
    pub meshes: Vec<Option<ChunkMesh>>,
}

impl ChunkMeshCache {
    pub fn new() -> Self {
        let chunk_mesh_cache = Self {
            meshes: vec![None; WORLD_VOLUME],
        };

        chunk_mesh_cache
    }

    pub fn insert(&mut self, chunk_id: ChunkID, mesh: ChunkMesh) {
        self.meshes[chunk_id] = Some(mesh);
    }

    pub fn get(&self, chunk_id: ChunkID) -> Option<&ChunkMesh> {
        self.meshes[chunk_id].as_ref()
    }

    pub fn needs_update(&self, chunk_id: ChunkID, last_update: u64) -> bool {
        self.meshes[chunk_id]
            .as_ref()
            .map_or(true, |mesh| mesh.last_render < last_update)
    }
}

#[derive(Debug)]
pub struct GpuChunkMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}

impl GpuChunkMesh {
    fn new(device: &wgpu::Device, mesh: &ChunkMesh) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("GpuChunkMesh Vertex Buffer"),
            contents: bytemuck::cast_slice(&mesh.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("GpuChunkMesh Index Buffer"),
            contents: bytemuck::cast_slice(&mesh.indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            vertex_buffer,
            index_buffer,
            index_count: mesh.indices.len() as u32,
        }
    }
}

#[derive(Debug, Default)]
pub struct GpuChunkMeshCache {
    pub meshes: HashMap<ChunkID, GpuChunkMesh>,
}

impl GpuChunkMeshCache {
    pub fn new() -> Self {
        let gpu_chunk_mesh_cache = Self {
            meshes: HashMap::default(),
        };

        gpu_chunk_mesh_cache
    }

    pub fn upload_mesh(&mut self, device: &wgpu::Device, chunk_id: ChunkID, mesh: &ChunkMesh) {
        let gpu_mesh = GpuChunkMesh::new(device, mesh);

        self.meshes.insert(chunk_id, gpu_mesh);
    }

    pub fn get(&self, chunk_id: ChunkID) -> Option<&GpuChunkMesh> {
        self.meshes.get(&chunk_id)
    }

    pub fn unload_chunk(&mut self, chunk_id: ChunkID) {
        self.meshes.remove(&chunk_id);
    }
}
