use crate::simulation::{chunk, observation::state_pair::StatePair, time::Tick};
use glam::IVec3;

#[derive(Clone, Debug)]
pub struct ChunkView {
    pub id: chunk::ID,
    pub tick: StatePair<Tick>,
    pub position: StatePair<IVec3>,
    pub mesh: StatePair<chunk::Mesh>,
}
