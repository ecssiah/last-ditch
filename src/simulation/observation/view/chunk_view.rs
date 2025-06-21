use crate::simulation::{observation::state_pair::StatePair, state::world::chunk};

#[derive(Clone, Default, Debug)]
pub struct ChunkView {
    pub id: chunk::ID,
    pub geometry: StatePair<chunk::Geometry>,
}
