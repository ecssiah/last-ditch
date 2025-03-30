#[derive(Debug, Default)]
pub struct Cache {
    meshes: HashMap<ChunkID, Mesh>,
}

impl Cache {
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