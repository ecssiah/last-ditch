use crate::simulation::{
    chunk,
    observation::{state_pair::StatePair, view::chunk_view::ChunkView},
    time::Tick,
};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct WorldView {
    pub tick: StatePair<Tick>,
    pub chunk_views: HashMap<chunk::ID, ChunkView>,
}
