use crate::simulation::state::world::{block, grid};
use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct FaceView {
    pub position: Vec3,
    pub direction: grid::Direction,
    pub block_kind: block::Kind,
}
