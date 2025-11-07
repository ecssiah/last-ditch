use crate::simulation::state::world::{block, grid};
use glam::IVec3;

#[derive(Clone, Debug)]
pub struct FaceView {
    pub position: IVec3,
    pub direction: grid::Direction,
    pub block_kind: block::Kind,
}
