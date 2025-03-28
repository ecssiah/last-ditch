use crate::simulation::{chunk::ChunkID, world::Tick};
use bytemuck::{Pod, Zeroable};
use std::collections::HashMap;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 4],
    pub ao: f32,
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x3,
        2 => Float32x4,
        3 => Float32,
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

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

#[derive(Debug, Default)]
pub struct MeshCache {
    meshes: HashMap<ChunkID, Mesh>,
}

impl MeshCache {
    pub fn new() -> Self {
        let mesh_cache = Self {
            meshes: HashMap::default(),
        };

        mesh_cache
    }

    pub fn needs_update(&self, chunk_id: ChunkID, last_update: Tick) -> bool {
        if let Some(mesh) = self.get(chunk_id) {
            mesh.last_update < last_update
        } else {
            true
        }
    }

    pub fn upload_mesh(
        &mut self,
        device: &wgpu::Device,
        chunk_id: ChunkID,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        last_update: Tick,
    ) {
        let mesh = Mesh::new(device, last_update, vertices, indices);

        self.meshes.insert(chunk_id, mesh);
    }

    pub fn get(&self, chunk_id: ChunkID) -> Option<&Mesh> {
        self.meshes.get(&chunk_id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ChunkID, &Mesh)> {
        self.meshes.iter()
    }

    pub fn values(&self) -> impl Iterator<Item = &Mesh> {
        self.meshes.values()
    }

    pub fn unload_chunk(&mut self, chunk_id: ChunkID) {
        self.meshes.remove(&chunk_id);
    }
}
