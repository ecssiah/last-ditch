use crate::simulation::{
    state::world::{block, cell},
    viewer::face_mask::FaceMask,
};

#[derive(Clone, Debug)]
pub struct BlockView {
    pub cell_id: cell::ID,
    pub block_kind: block::Kind,
    pub face_mask: FaceMask,
}
