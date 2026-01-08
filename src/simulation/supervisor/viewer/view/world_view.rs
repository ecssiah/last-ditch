use crate::simulation::{
    state::world::sector::sector_index::SectorIndex, supervisor::viewer::SectorView,
};
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct WorldView {
    pub active: bool,
    pub sector_view_map: HashMap<SectorIndex, SectorView>,
}
