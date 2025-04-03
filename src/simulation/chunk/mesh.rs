use crate::simulation::chunk;

#[derive(Clone, Debug, Default)]
pub struct Mesh {
    pub vertices: Vec<chunk::Vertex>,
    pub indices: Vec<u32>,
}
