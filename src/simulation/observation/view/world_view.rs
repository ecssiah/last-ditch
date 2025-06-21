use crate::simulation::{observation::view::chunk_view::ChunkView, state::world::chunk};
use std::collections::HashMap;

#[derive(Clone, Default, Debug)]
pub struct WorldView {
    pub chunk_view_map: HashMap<chunk::ID, ChunkView>,
}
