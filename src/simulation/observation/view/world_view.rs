use crate::simulation::{self, observation::view::chunk_view::ChunkView, state::world::{chunk, grid::Grid}};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct WorldView {
    pub grid: Grid,
    pub chunk_view_map: HashMap<chunk::ID, ChunkView>,
}

impl WorldView {
    pub fn new() -> Self {
        Self {
            grid: Grid::new(simulation::Kind::Empty),
            chunk_view_map: HashMap::new(),
        }
    }
}
