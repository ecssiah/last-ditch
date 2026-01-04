use crate::simulation::supervisor::viewer::SectorView;
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct WorldView {
    pub active: bool,
    pub sector_view_map: HashMap<usize, SectorView>,
}
