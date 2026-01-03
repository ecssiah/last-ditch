use crate::simulation::overseer::viewer::SectorView;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct WorldView {
    pub sector_view_map: HashMap<usize, SectorView>,
}

impl WorldView {
    pub fn new() -> Self {
        Self {
            sector_view_map: HashMap::default(),
        }
    }

    pub fn default() -> Self {
        Self::new()
    }
}
