use crate::simulation::{observation::state_pair::StatePair, world::chunk};

#[derive(Clone, Debug)]
pub struct ChunkView {
    pub id: chunk::ID,
    pub geometry: StatePair<chunk::Geometry>,
}
