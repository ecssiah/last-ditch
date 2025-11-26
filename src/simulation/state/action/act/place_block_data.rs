use crate::simulation::state::world::block;

#[derive(Clone, Copy, Debug)]
pub struct PlaceBlockData {
    pub block_kind: block::Kind,
}
