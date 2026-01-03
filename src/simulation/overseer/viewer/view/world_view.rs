use crate::simulation::overseer::viewer::SectorView;
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct WorldView {
    pub sector_view_map: HashMap<usize, SectorView>,
}
