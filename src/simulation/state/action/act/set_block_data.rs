use ultraviolet::IVec3;
use crate::simulation::state::world::block;

#[derive(Clone, Copy, Debug)]
pub struct SetBlockData {
    pub position: IVec3,
    pub block_kind: block::Kind,
}
