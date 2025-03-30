#[derive(Debug, Default, Clone)]
pub struct Mesh {
    pub vertices: Vec<crate::simulation::chunk::vertex::Vertex>,
    pub indices: Vec<u32>,
}
