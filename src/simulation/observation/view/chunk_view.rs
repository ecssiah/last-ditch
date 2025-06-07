use crate::simulation::{observation::state_pair::StatePair, time::Tick, world::chunk};

#[derive(Clone, Debug)]
pub struct ChunkView {
    pub tick: StatePair<Tick>,
    pub id: chunk::ID,
    pub geometry: StatePair<chunk::Geometry>,
}
