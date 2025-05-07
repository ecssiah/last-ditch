use crate::simulation::{observation::state_pair::StatePair, time::Tick, world::chunk};
use glam::IVec3;

#[derive(Clone, Debug)]
pub struct ChunkView {
    pub tick: StatePair<Tick>,
    pub id: chunk::ID,
    pub position: StatePair<IVec3>,
    pub geometry: StatePair<chunk::Geometry>,
}
