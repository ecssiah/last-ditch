use crate::simulation::{manager::viewer::SectorView, state::world::sector};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct WorldView {
    pub sector_view_map: HashMap<sector::ID, SectorView>,
}

impl WorldView {
    pub fn new() -> Self {
        Self {
            sector_view_map: HashMap::new(),
        }
    }
}
